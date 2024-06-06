pub mod actions;
pub mod settings;
pub mod types;

pub use actions::BussAction;
pub use settings::BussSettings;

/// Version of bussin binary protocol
mod version {
    pub const MAJOR: u8 = 1;
    pub const MINOR: u8 = 0;
}

/// The protocol header
pub struct BussHeader {
    /// Set to 0x00042069. The server/client should check if the first 4 bytes matches these sequence. Otherwise should cancel the request and respond with an error.
    /// If you see this, we are bussin.
    pub magic_number: u32,
    /// Bussin protocol version number. e.g. if currently at version 69.1 it will be the 69 part
    pub version_major: u8,
    /// Bussing protocol version number minor. e.g. if currently at version 69.1 it will be the 1 part
    pub version_minor: u8,
    /// Action the client wants executed or what response code the server replied with.
    pub action: BussAction,
    /// Not used for now
    pub padding: u8,
}

/// Trait to facilitate conversion of various bussin types to raw bytes
pub trait ToBytes {
    /// Convert to bytes
    fn to_bytes(self) -> Vec<u8>;
}

/// Trait to do the conversion of raw bytes to the given type
pub trait FromBytes<T: Sized> {
    /// get a type from the sized byte array
    fn from_bytes(bytes: T) -> Self;
}

impl BussHeader {
    /// Create new head
    pub fn new() -> Self {
        BussHeader {
            magic_number: 0x00042069,
            version_major: version::MAJOR,
            version_minor: version::MINOR,
            action: BussAction::Noop,
            padding: 0,
        }
    }
    /// Sets the action to be performed
    pub fn set_action(&mut self, action: BussAction) {
        self.action = action;
    }
}

impl Default for BussHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl ToBytes for BussHeader {
    fn to_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.magic_number.to_be_bytes());
        bytes.push(self.version_major);
        bytes.push(self.version_minor);
        bytes.push(self.action as u8);
        bytes.push(self.padding);
        bytes
    }
}
impl FromBytes<&[u8; 8]> for BussHeader {
    fn from_bytes(bytes: &[u8; 8]) -> Self {
        let magic_bytes = &bytes[0..4];
        let magic_number = u32::from_be_bytes(magic_bytes.try_into().unwrap());
        let version_major = bytes[4];
        let version_minor = bytes[5];
        let action_number = bytes[6];
        let action: BussAction = action_number.try_into().unwrap();
        let padding = bytes[7];

        BussHeader {
            magic_number,
            version_major,
            version_minor,
            action,
            padding,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{self, Write},
    };

    use crate::{
        settings::{BodyLength, Settings},
        types::BussPath,
        BussAction, BussHeader, ToBytes,
    };

    #[test]
    fn skeleton() -> io::Result<()> {
        let mut header = BussHeader::new();
        header.set_action(BussAction::Read);
        let mut settings = Settings::new();
        settings.add(Box::new(BodyLength::new(25)));

        let path = BussPath::new("/");

        let mut file: File = File::create("test.bbp")?;
        let _ = file.write(&header.to_bytes())?;
        let _ = file.write(&path.to_bytes())?;
        let _ = file.write(&settings.to_bytes())?;
        Ok(())
    }
}
