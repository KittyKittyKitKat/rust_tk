use std::{error::Error as stdError, fmt, io::Error as ioError, sync::mpsc::SendError};

/// Interpreter Errors
#[derive(Debug, Clone)]
pub enum InterpreterError {
    DeadInterpreter,
    MalformedCommand,
    FailedCommand,
    FailedCommandResult,
}

impl stdError for InterpreterError {}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DeadInterpreter => write!(f, "Tried to interact with a dead interpreter."),
            Self::MalformedCommand => write!(f, "Malformed command."),
            Self::FailedCommand => write!(f, "Command failed to execute."),
            Self::FailedCommandResult => write!(f, "Failed to fetch result of command."),
            // Self::FailedToStart => write!(f, "Failed to start the interpreter"),
        }
    }
}

impl<T> From<SendError<T>> for InterpreterError {
    fn from(_: SendError<T>) -> Self {
        Self::FailedCommand
    }
}

impl From<ioError> for InterpreterError {
    fn from(_: ioError) -> Self {
        Self::FailedCommandResult
    }
}
