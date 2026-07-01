//! Sharp SM83 CPU implementation

use crate::{
    bus::Bus,
    cycles::MachineCycles,
    errors::CpuError,
    instructions::{self, Instruction, misc::Misc},
    interrupts::next_pending_interrupt,
    registers::{Registers, SingleRegister},
};

#[allow(non_snake_case, clippy::struct_excessive_bools)]
#[derive(Debug, PartialEq, Eq)]
pub struct CpuFlags {
    /// Interrupt Master Enable
    ///
    /// Enables interrupts based on the current state of the IE register (memory address 0xFFFF)
    pub IME: bool,

    /// If true at the start of a machine cycle IME should be enabled
    pub IME_scheduled: bool,

    /// The CPU is in halted mode, the bus timer continues, but PC is not incremented.
    pub halted: bool,

    /// Used to handle the HALT bug where PC is not incremented correctly when IME=0 and there
    /// is a pending interrupt.
    pub skip_pc_increment: bool,
}

impl Default for CpuFlags {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuFlags {
    #[must_use]
    pub fn new() -> Self {
        Self {
            IME: false,
            IME_scheduled: false,
            halted: false,
            skip_pc_increment: false,
        }
    }
}

pub struct CPU {
    flags: CpuFlags,
    out: Option<Box<dyn std::io::Write>>,
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    #[must_use]
    pub fn new() -> Self {
        Self {
            flags: CpuFlags::new(),
            out: None,
        }
    }

    #[must_use]
    pub fn new_with_trace(out: Box<dyn std::io::Write>) -> Self {
        Self {
            flags: CpuFlags::new(),
            out: Some(out),
        }
    }

    pub fn tick(
        &mut self,
        registers: &mut Registers,
        bus: &mut Bus,
    ) -> Result<(Instruction, MachineCycles), CpuError> {
        let (instruction, mut cycles) = if self.flags.halted {
            (Instruction::Misc(Misc::HALT()), MachineCycles::new(1))
        } else {
            let pending_interrupt = bus.has_pending_interrupt();
            let opcode = self.fetch(registers, bus);
            let instruction = self.decode(opcode, registers, bus)?;
            let cycles = self.execute(&instruction, registers, bus)?;

            // HALT PC increment bug
            self.flags.skip_pc_increment = instruction == Instruction::Misc(Misc::HALT())
                && !self.flags.IME
                && pending_interrupt;

            (instruction, cycles)
        };

        bus.tick(cycles);

        if self.flags.halted && bus.has_pending_interrupt() {
            self.flags.halted = false;
        }

        cycles += self.maybe_handle_interrupt(registers, bus);

        Ok((instruction, cycles))
    }

    pub fn fetch(&mut self, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let opcode = bus.get(registers.PC);
        if !self.flags.skip_pc_increment {
            registers.PC += 1;
        }
        opcode
    }

    pub fn decode(
        &self,
        opcode: u8,
        registers: &mut Registers,
        bus: &Bus,
    ) -> Result<Instruction, CpuError> {
        let instruction = instructions::decode(opcode, registers.PC, bus)?;
        registers.PC += instruction.length() - 1; // instruction length - opcode length

        Ok(instruction)
    }

    pub fn execute(
        &mut self,
        instruction: &Instruction,
        registers: &mut Registers,
        bus: &mut Bus,
    ) -> Result<MachineCycles, CpuError> {
        if let Some(out) = self.out.as_mut() {
            Self::gameboy_doctor_output(out, registers, bus);
        }

        if self.flags.IME_scheduled {
            self.flags.IME = true;
            self.flags.IME_scheduled = false;
        }

        let cycles = instruction.execute(registers, bus, &mut self.flags)?;

        Ok(cycles)
    }

    #[allow(unused)]
    fn gameboy_doctor_output(w: &mut Box<dyn std::io::Write>, registers: &Registers, bus: &Bus) {
        let pc = registers.PC;
        writeln!(
            w,
            "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
            registers.get_single(&SingleRegister::A),
            registers.get_single(&SingleRegister::F),
            registers.get_single(&SingleRegister::B),
            registers.get_single(&SingleRegister::C),
            registers.get_single(&SingleRegister::D),
            registers.get_single(&SingleRegister::E),
            registers.get_single(&SingleRegister::H),
            registers.get_single(&SingleRegister::L),
            registers.SP,
            registers.PC,
            bus.get(pc),
            bus.get(pc + 1),
            bus.get(pc + 2),
            bus.get(pc + 3),
        );
    }

    fn maybe_handle_interrupt(
        &mut self,
        registers: &mut Registers,
        bus: &mut Bus,
    ) -> MachineCycles {
        if self.flags.IME
            && let Some(interrupt) = next_pending_interrupt(bus)
        {
            // Reset
            self.flags.IME = false;
            bus.clear_interrupt(interrupt);

            bus.tick(MachineCycles::new(2)); // Wait two M-cycles

            let sp = registers.decrement_sp();
            bus.set_u16(sp, registers.PC);
            registers.PC = interrupt.vector();

            bus.tick(MachineCycles::new(3));

            MachineCycles::new(5)
        } else {
            MachineCycles::new(0)
        }
    }
}

#[allow(clippy::unusual_byte_groupings)]
#[cfg(test)]
mod test {
    use crate::bus::{ADDR_IE, ADDR_IF, ADDR_TAC, ADDR_TIMA, ADDR_TMA, Clock, MASK_TIMER_ENABLED};
    use crate::instructions::Condition;
    use crate::interrupts::Interrupt;
    use crate::registers::DoubleRegister;

    use super::*;
    use instructions::Instruction;
    use instructions::control_flow;
    use instructions::misc;

    #[test]
    fn cpu_tick_executes_instructiona() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        let mut cpu = CPU::new();

        let noop = 0b0000_0000;
        bus.set_u16(0x0000, noop);

        assert_eq!(0, registers.PC);

        let (instruction, _) = cpu.tick(&mut registers, &mut bus).unwrap();

        assert_eq!(Instruction::Misc(misc::Misc::NOP()), instruction);
        assert_eq!(instruction.length(), registers.PC);
    }

    #[test]
    fn halt_ime_true_basic_scenario_works() {
        let halt = 0b0111_0110;
        let mut registers = Registers::new();
        let mut bus = Bus::new().with_memory(&[
            (0x0000, halt),
            (ADDR_IE, Interrupt::Timer.mask()),
            (ADDR_TIMA, 0xFF),
            (ADDR_TMA, 0xAB),
            (ADDR_TAC, MASK_TIMER_ENABLED | u8::from(Clock::T16)),
        ]);
        let mut cpu = CPU::new();
        cpu.flags.IME = true;

        assert_eq!(0, registers.PC);
        assert!(!cpu.flags.halted);

        let (instruction, _) = cpu.tick(&mut registers, &mut bus).unwrap();

        assert_eq!(Instruction::Misc(Misc::HALT()), instruction);
        assert_eq!(1, registers.PC);
        assert!(cpu.flags.halted);

        for _ in 0..3 {
            let (instruction, _) = cpu.tick(&mut registers, &mut bus).unwrap();
            assert_eq!(Instruction::Misc(Misc::HALT()), instruction);
            assert_eq!(1, registers.PC);
            assert!(cpu.flags.halted);
        }

        let _ = cpu.tick(&mut registers, &mut bus).unwrap();

        assert_eq!(0x50, registers.PC);
        assert!(!cpu.flags.halted);
    }

    #[test]
    fn halt_ime_false_basic_scenario_works() {
        let halt = 0b0111_0110;
        let mut registers = Registers::new();
        let mut bus = Bus::new().with_memory(&[
            (0x0000, halt),
            (ADDR_IE, Interrupt::Timer.mask()),
            (ADDR_TIMA, 0xFF),
            (ADDR_TMA, 0xAB),
            (ADDR_TAC, MASK_TIMER_ENABLED | u8::from(Clock::T16)),
        ]);
        let mut cpu = CPU::new();
        cpu.flags.IME_scheduled = false;
        cpu.flags.IME = false;

        assert_eq!(0, registers.PC);
        assert!(!cpu.flags.halted);

        let (instruction, _) = cpu.tick(&mut registers, &mut bus).unwrap();

        assert_eq!(Instruction::Misc(Misc::HALT()), instruction);
        assert_eq!(1, registers.PC);
        assert!(cpu.flags.halted);

        for _ in 0..3 {
            let (instruction, _) = cpu.tick(&mut registers, &mut bus).unwrap();
            assert_eq!(Instruction::Misc(Misc::HALT()), instruction);
            assert_eq!(1, registers.PC);
            assert!(cpu.flags.halted);
        }

        let _ = cpu.tick(&mut registers, &mut bus).unwrap();

        assert_eq!(0x0001, registers.PC);
        assert!(!cpu.flags.halted);
    }

    #[test]
    fn halt_bug_fails_to_increment_pc_on_next_fetch() {
        let halt = 0b0111_0110;
        let inc_hl = 0b0011_0100;
        let hl_addr = 0xABCD;
        let mut registers = Registers::new();
        registers.set_double(&DoubleRegister::HL, hl_addr);
        let mut bus = Bus::new().with_memory(&[
            (0x0000, halt),
            (0x0001, inc_hl),
            (hl_addr, 10),
            (ADDR_IE, Interrupt::VBlank.mask()),
            (ADDR_IF, Interrupt::VBlank.mask()),
        ]);
        let mut cpu = CPU::new();
        cpu.flags.IME = false;
        cpu.flags.IME_scheduled = false;
        cpu.flags.skip_pc_increment = false;

        let _ = cpu.tick(&mut registers, &mut bus).unwrap();
        assert_eq!(0x0001, registers.PC);
        assert_eq!(10, bus.get(hl_addr));
        assert!(cpu.flags.skip_pc_increment);

        let _ = cpu.tick(&mut registers, &mut bus).unwrap();
        assert_eq!(0x0001, registers.PC);
        assert_eq!(11, bus.get(hl_addr));
        assert!(!cpu.flags.skip_pc_increment);

        let _ = cpu.tick(&mut registers, &mut bus).unwrap();
        assert_eq!(0x0002, registers.PC);
        assert_eq!(12, bus.get(hl_addr));
        assert!(!cpu.flags.skip_pc_increment);
    }

    #[test]
    fn halt_bug_fails_to_increment_pc_on_next_fetch_2() {
        let halt = 0b0111_0110;
        let add_n = 0b1100_0110;
        let operand = 1;
        let mut registers = Registers::new();
        registers.set_single(&SingleRegister::A, 10);
        let mut bus = Bus::new().with_memory(&[
            (0x0000, halt),
            (0x0001, add_n),
            (0x0002, operand),
            (ADDR_IE, Interrupt::VBlank.mask()),
            (ADDR_IF, Interrupt::VBlank.mask()),
        ]);
        let mut cpu = CPU::new();
        cpu.flags.IME = false;
        cpu.flags.IME_scheduled = false;
        cpu.flags.skip_pc_increment = false;

        let _ = cpu.tick(&mut registers, &mut bus).unwrap();
        assert_eq!(0x0001, registers.PC);
        assert_eq!(10, registers.get_single(&SingleRegister::A));
        assert!(cpu.flags.skip_pc_increment);

        let _ = cpu.tick(&mut registers, &mut bus).unwrap();
        assert_eq!(0x0002, registers.PC);
        assert_eq!(10 + add_n, registers.get_single(&SingleRegister::A));
        assert!(!cpu.flags.skip_pc_increment);

        let _ = cpu.tick(&mut registers, &mut bus).unwrap();
        assert_eq!(0x0005, registers.PC);
        assert_eq!(10 + add_n, registers.get_single(&SingleRegister::A));
        assert!(!cpu.flags.skip_pc_increment);
    }

    #[test]
    fn cpu_tick_handles_interrupt_scheduling() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        let mut cpu = CPU::new();

        let ei_op = 0b1111_1011;
        let noop = 0b0000_0000;

        bus.set_u16(0x0000, ei_op);
        bus.set_u16(0x0002, noop);

        let (instruction, _) = cpu
            .tick(&mut registers, &mut bus)
            .expect("Failed to execute EI instruction");

        assert_eq!(Instruction::Misc(misc::Misc::EI()), instruction);
        assert_eq!(instruction.length(), registers.PC);
        assert_eq!(
            CpuFlags {
                IME: false,
                IME_scheduled: true,
                ..Default::default()
            },
            cpu.flags
        );

        cpu.tick(&mut registers, &mut bus)
            .expect("Failed to execute Noop instruction");

        assert_eq!(
            CpuFlags {
                IME: true,
                IME_scheduled: false,
                ..Default::default()
            },
            cpu.flags
        );
    }

    #[test]
    fn pc_remains_unchanged_after_jr_fe_instructions() {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        let mut cpu = CPU::new();

        let jr_fe = 0xFE_18; // as BE bytes

        let prev_pc = registers.PC;

        bus.set_u16(0x0000, jr_fe);

        let (instruction, _) = cpu.tick(&mut registers, &mut bus).unwrap();

        assert_eq!(
            Instruction::ControlFlow(control_flow::ControlFlow::JR(0xFE)),
            instruction
        );
        assert_eq!(prev_pc, registers.PC);
    }

    #[test]
    fn retc_instruction_works() {
        fn test_retc_instruction(instruction_byte: u8, cond: Condition) {
            let sp = 0x1234;
            let location = 0xAABB;

            let (mut registers, mut memory, mut cpu) =
                setup_conditional_instruction(instruction_byte, Operand::None);

            match cond {
                Condition::Carry => registers.set_carry(true),
                Condition::NoCarry => registers.set_carry(false),
                Condition::Zero => registers.set_zero(true),
                Condition::NotZero => registers.set_zero(false),
            }

            registers.SP = sp; // Update SP
            memory.set_u16(sp, location); // Update SP pointer

            let (instruction, _) = cpu.tick(&mut registers, &mut memory).unwrap();

            assert_eq!(
                Instruction::ControlFlow(control_flow::ControlFlow::RETC(cond)),
                instruction
            );
            assert_eq!(location, registers.PC);

            let (mut registers, mut memory, mut cpu) =
                setup_conditional_instruction(instruction_byte, Operand::None);

            match cond {
                Condition::Carry => registers.set_carry(false),
                Condition::NoCarry => registers.set_carry(true),
                Condition::Zero => registers.set_zero(false),
                Condition::NotZero => registers.set_zero(true),
            }

            registers.SP = sp; // Update SP!
            memory.set_u16(sp, location); // Update SP pointer

            let prev_pc = registers.PC;

            let (instruction, _) = cpu.tick(&mut registers, &mut memory).unwrap();

            assert_eq!(
                Instruction::ControlFlow(control_flow::ControlFlow::RETC(cond)),
                instruction
            );
            assert_eq!(prev_pc + instruction.length(), registers.PC);
        }

        test_retc_instruction(0b110_00_000, Condition::NotZero);
        test_retc_instruction(0b110_01_000, Condition::Zero);
        test_retc_instruction(0b110_10_000, Condition::NoCarry);
        test_retc_instruction(0b110_11_000, Condition::Carry);
    }

    #[test]
    fn instruction_priority_works() {
        let pending_interrupts = Interrupt::VBlank.mask() | Interrupt::Timer.mask();
        let mut registers = Registers::new();
        let mut bus =
            Bus::new().with_memory(&[(ADDR_IE, pending_interrupts), (ADDR_IF, pending_interrupts)]);
        let mut cpu = CPU::new();
        cpu.flags.IME = true;

        let _ = cpu.tick(&mut registers, &mut bus).unwrap();

        assert_eq!(Interrupt::VBlank.vector(), registers.PC);
        assert_eq!(0, bus.get(ADDR_IF) & Interrupt::VBlank.mask());
        assert!(bus.get(ADDR_IF) & Interrupt::Timer.mask() != 0);
    }

    #[test]
    fn jrc_instruction_works() {
        fn test_jrc_instruction(instruction_byte: u8, cond: Condition) {
            let operand = 20;

            let (mut registers, mut memory, mut cpu) =
                setup_conditional_instruction(instruction_byte, Operand::U8(operand));

            match cond {
                Condition::Carry => registers.set_carry(true),
                Condition::NoCarry => registers.set_carry(false),
                Condition::Zero => registers.set_zero(true),
                Condition::NotZero => registers.set_zero(false),
            }

            let prev_pc = registers.PC;

            let (instruction, _) = cpu.tick(&mut registers, &mut memory).unwrap();

            assert_eq!(
                Instruction::ControlFlow(control_flow::ControlFlow::JRC(operand, cond)),
                instruction
            );
            assert_eq!(
                prev_pc + u16::from(operand) + instruction.length(),
                registers.PC
            );

            let (mut registers, mut memory, mut cpu) =
                setup_conditional_instruction(instruction_byte, Operand::U8(operand));

            match cond {
                Condition::Carry => registers.set_carry(false),
                Condition::NoCarry => registers.set_carry(true),
                Condition::Zero => registers.set_zero(false),
                Condition::NotZero => registers.set_zero(true),
            }

            let prev_pc = registers.PC;

            let (instruction, _) = cpu.tick(&mut registers, &mut memory).unwrap();

            assert_eq!(
                Instruction::ControlFlow(control_flow::ControlFlow::JRC(operand, cond)),
                instruction
            );
            assert_eq!(prev_pc + instruction.length(), registers.PC,);
        }

        test_jrc_instruction(0b001_00_000, Condition::NotZero);
        test_jrc_instruction(0b001_01_000, Condition::Zero);
        test_jrc_instruction(0b001_10_000, Condition::NoCarry);
        test_jrc_instruction(0b001_11_000, Condition::Carry);
    }

    #[test]
    fn jpc_instruction_works() {
        fn test_jpc_instruction(instruction_byte: u8, cond: Condition) {
            let operand = 0xAABB;

            let (mut registers, mut memory, mut cpu) =
                setup_conditional_instruction(instruction_byte, Operand::U16(operand));

            match cond {
                Condition::Carry => registers.set_carry(true),
                Condition::NoCarry => registers.set_carry(false),
                Condition::Zero => registers.set_zero(true),
                Condition::NotZero => registers.set_zero(false),
            }

            let (instruction, _) = cpu.tick(&mut registers, &mut memory).unwrap();

            assert_eq!(
                Instruction::ControlFlow(control_flow::ControlFlow::JPC(operand, cond)),
                instruction
            );
            assert_eq!(operand, registers.PC);

            let (mut registers, mut memory, mut cpu) =
                setup_conditional_instruction(instruction_byte, Operand::U16(operand));

            match cond {
                Condition::Carry => registers.set_carry(false),
                Condition::NoCarry => registers.set_carry(true),
                Condition::Zero => registers.set_zero(false),
                Condition::NotZero => registers.set_zero(true),
            }

            let prev_pc = registers.PC;

            let (instruction, _) = cpu.tick(&mut registers, &mut memory).unwrap();

            assert_eq!(
                Instruction::ControlFlow(control_flow::ControlFlow::JPC(operand, cond)),
                instruction
            );
            assert_eq!(prev_pc + instruction.length(), registers.PC,);
        }

        test_jpc_instruction(0b110_00_010, Condition::NotZero);
        test_jpc_instruction(0b110_01_010, Condition::Zero);
        test_jpc_instruction(0b110_10_010, Condition::NoCarry);
        test_jpc_instruction(0b110_11_010, Condition::Carry);
    }

    #[derive(Copy, Clone)]
    enum Operand {
        None,
        U8(u8),
        U16(u16),
    }

    fn setup_conditional_instruction(instruction: u8, operand: Operand) -> (Registers, Bus, CPU) {
        let mut registers = Registers::new();
        let mut bus = Bus::new();
        let cpu = CPU::new();

        registers.reset();
        bus.reset();

        let location = registers.PC;
        bus.set(location, instruction);

        match operand {
            Operand::None => {}
            Operand::U8(x) => {
                bus.set(location + 1, x);
            }
            Operand::U16(x) => {
                bus.set_u16(location + 1, x);
            }
        }

        (registers, bus, cpu)
    }
}
