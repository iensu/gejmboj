use std::{error::Error, fmt::Display};

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Io(std::io::Error),
    Cpu(cpu::errors::CpuError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(f, "{error}"),
            Self::Cpu(cpu_error) => write!(f, "{cpu_error}"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Cpu(cpu_error) => Some(cpu_error),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<cpu::errors::CpuError> for AppError {
    fn from(value: cpu::errors::CpuError) -> Self {
        Self::Cpu(value)
    }
}
