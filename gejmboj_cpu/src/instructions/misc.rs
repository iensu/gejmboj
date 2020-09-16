use crate::instruction_group;

instruction_group! {
    /// Miscelleneous instructions
    Misc (_registers, _memory, _cpu_flags) {

        /// No operation
        Noop() [1] => {
            Ok(1)
        }
    }
}
