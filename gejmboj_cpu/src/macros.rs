//! Macros used within this crate

/// Macro to define a group of instructions
#[macro_export]
macro_rules! instruction_group {
    ( $(#[$groupdocs:meta])
      *$group_name:ident ($r:ident, $m:ident, $c:ident) {
          $($(#[$itemdocs:meta])*
            $item_name:ident($($operand:ident: $t:tt),*) [ $length:literal ] => $execute:block)+
      }) => {

        $(#[$groupdocs])*
        #[derive(Debug, PartialEq)]
        pub enum $group_name {
            $($(#[$itemdocs])*$item_name($($t),*),)+
        }

        impl $group_name {
            pub fn execute(&self,
                           $r: &mut $crate::registers::Registers,
                           $m: &mut $crate::memory::Memory,
                           $c: &mut $crate::cpu::CpuFlags
            ) -> $crate::instructions::InstructionResult {
                match self {
                    $($group_name::$item_name($($operand),*) => $execute,)+
                }
            }

            pub fn length(&self) -> u16 {
                match self {
                    $($group_name::$item_name($($operand),*) => {
                        $(drop($operand);)*
                        $length
                    },)+
                }
            }
        }
    }
}

#[cfg(test)]
#[doc(hidden)]
#[macro_export]
macro_rules! instruction_tests {
    ($($testname:ident ($r:ident, $m:ident, $c:ident) => $testbody:block)*) => {
        #[cfg(test)]
        mod instruction_tests {
            use super::*;
            #[allow(unused_imports)]
            use $crate::registers::*;

            $(
                #[test]
                fn $testname() {
                    let mut $r = Registers::new();
                    let mut $m = $crate::memory::Memory::new();
                    let mut $c = $crate::cpu::CpuFlags::new();

                    $testbody
                }
            )*
        }
    }
}

/// Combines instructions into a single enum
#[doc(hidden)]
#[macro_export]
macro_rules! combine_instructions {
    ($name:ident( $($group:ident),+ )) => {
        #[derive(Debug, PartialEq)]
        pub enum $name {
            $($group($group)),+
        }

        impl $name {
            pub fn execute(
                &self,
                mut registers: &mut $crate::registers::Registers,
                mut memory: &mut $crate::memory::Memory,
                mut cpu_flags: &mut $crate::cpu::CpuFlags,
            ) -> InstructionResult {
                match self {
                    $($name::$group(instr) => instr.execute(&mut registers, &mut memory, &mut cpu_flags)),+
                }
            }

            pub fn length(&self) -> u16 {
                match self {
                    $($name::$group(instr) => instr.length()),+
                }
            }
        }
    };
}
