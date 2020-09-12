use crate::define_instruction;

define_instruction! {
    /// No operation
    Noop { "NOOP"; 1 }

    (self) => {
        Ok(1)
    }
}
