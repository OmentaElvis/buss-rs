use crate::ToBytes;

/// Settings are values passed during both request and response
/// that are supposed to alter the default behavior of the client/server.
/// They can also carry extra information for the protocol.
/// The settings type/flag are of type u8 (1 byte).
/// This means there are 256 possible settings.
/// Values from 0 to 254(0xfe) are reserved for standard settings values.
/// For your custom settings, value 0xff(255) indicates custom settings.
#[repr(u8)]
#[derive(Copy, Clone)]
pub enum BussSettings {
    /// The number of bytes contained in body content
    BodyLength = 0,
    /// The host domain name that this request was requested for
    Host,
    /// The custom tag
    Custom = 0xff,
}

/// Provides easy conversion of various settings type into binary data
pub trait BaseSettings {
    /// Get the BussSettings type this setting represents
    fn get_type(&self) -> BussSettings;
    /// Get the body in bytes of this settings
    fn get_body(&self) -> Vec<u8>;
}

/// User defined settings in byte form, ready to be passed to an encoder/decoder
pub struct CustomSettings {
    /// The custom type
    custom_tag: u8,
    /// The bytes of the custom settings
    bytes: Vec<u8>,
}

impl CustomSettings {
    /// New settings in byte form
    pub fn new(custom_tag: u8, bytes: Vec<u8>) -> Self {
        CustomSettings { custom_tag, bytes }
    }
}

impl BaseSettings for CustomSettings {
    fn get_type(&self) -> BussSettings {
        BussSettings::Custom
    }
    fn get_body(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push(self.get_type() as u8);
        bytes.push(self.custom_tag);
        bytes.extend(self.bytes.iter());
        bytes
    }
}

/// Holds the list of settings to be written to the byte stream
pub struct Settings {
    /// List of settings
    entries: Vec<Box<dyn BaseSettings>>,
}

impl Settings {
    /// Creates a new settings pool
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// add a settings
    pub fn add(&mut self, entry: Box<dyn BaseSettings>) {
        self.entries.push(entry);
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Box<dyn BaseSettings>> {
        self.entries.iter()
    }
}

impl ToBytes for Settings {
    fn to_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        // Write length
        let length = self.entries.len() as u16;
        bytes.extend(length.to_be_bytes());
        for s in self.entries {
            bytes.push(s.get_type() as u8);
            bytes.append(&mut s.get_body());
        }

        bytes
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

/// BodyLength header/settings
/// Represents the number of bytes in the body of respose/request
pub struct BodyLength {
    /// Number of bodies
    length: usize,
}

impl BodyLength {
    /// New body length
    pub fn new(size: usize) -> Self {
        BodyLength { length: size }
    }
    /// Returns the length of body
    pub fn get_length(&self) -> usize {
        self.length
    }
}

impl BaseSettings for BodyLength {
    fn get_type(&self) -> BussSettings {
        BussSettings::BodyLength
    }
    fn get_body(&self) -> Vec<u8> {
        self.length.to_be_bytes().to_vec()
    }
}
