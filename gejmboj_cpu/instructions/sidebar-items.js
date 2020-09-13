initSidebarItems({"enum":[["Condition",""]],"fn":[["decode","Decode an operation code into an `Instruction`."]],"struct":[["Call","Unconditional call of the function at operand address."],["CallIf","Conditional function call."],["Jp","Unconditional jump to location specified by 16-bit operand."],["JpIf","Conditional jump to location specified by 16-bit operand."],["JpToHL","Unconditional jump to location specified by register HL"],["JpToOffset","Unconditional jump to location at current + offset"],["JpToOffsetIf","Conditional jump to relative address specified by offset operand."],["Ld","Loads data from register `r2` into `r1`."],["LdByte","Loads `operand` into register `r`."],["LdByteToHL","Load the value of `operand` into the location pointed to by `HL`"],["LdFromHL","Loads data pointed to by HL into `r`."],["LdToHL","Loads data in `r` into location pointed to by HL."],["Noop","No operation"],["Ret","Unconditional return from function."],["RetI","Unconditional return from a function which enables interrupts"],["RetIf","Conditionally return from function."],["Rst","Unconditional function call to the RESET address defined by bits 3-5"]],"trait":[["Instruction","Trait for implementing a Sharp SM83 instruction."]],"type":[["InstructionResult","Return either the number of consumed machine cycles, or a `CpuError`."]]});