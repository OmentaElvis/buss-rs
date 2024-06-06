use std::fmt::Display;

/// This is an enum of values that tell the server what action
/// to perform and the client what was the response code of the
/// action performed. There are possible 256 different actions possible and response codes of between 0 and 255.
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum BussAction {
    /// do nothing
    Noop = 0,
    /// Read request
    Read,
    /// Write request
    Write,
    /// Modify request
    Modify,
    /// Delete request
    Delete,
}

/// Parsing error for buss action
#[derive(Debug)]
pub enum BussActionError {
    /// Unexpected value was encountered which can not be handled internally
    Unhandled(u8),
}
impl Display for BussActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unhandled(value) => write!(
                f,
                "Unexpected action value 0x{:x} which has not been implemented internally.",
                value
            ),
        }
    }
}

impl TryFrom<u8> for BussAction {
    type Error = BussActionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(BussAction::Noop),
            1 => Ok(BussAction::Read),
            2 => Ok(BussAction::Write),
            3 => Ok(BussAction::Modify),
            4 => Ok(BussAction::Delete),
            _ => Err(BussActionError::Unhandled(value)),
        }
    }
}
