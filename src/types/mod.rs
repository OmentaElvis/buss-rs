use crate::ToBytes;

/// String representing what path the request is for
pub struct BussPath {
    path: String,
}

impl BussPath {
    /// creates a new path
    pub fn new(path: &str) -> Self {
        Self {
            path: String::from(path),
        }
    }
}

impl ToBytes for BussPath {
    fn to_bytes(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.path.len().to_be_bytes().iter());
        bytes.extend(self.path.into_bytes().iter());
        bytes
    }
}
