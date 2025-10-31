// hex + bytes
#[derive(Debug, Clone, Default)]
pub struct TransportPair {
    pub hex: String,
    pub bytes: Vec<u8>,
}

impl TransportPair {
    pub fn new(hex: String, bytes: Vec<u8>) -> Self {
        Self { hex, bytes }
    }

    pub fn set_hex(&mut self, hex: &str) {
        self.hex = hex.into();
    }

    pub fn set_bytes(&mut self, bytes: &[u8]) {
        self.bytes = bytes.into();
    }

    pub fn get_hex_clone(&self) -> String {
        self.hex.clone()
    }

    pub fn get_bytes_clone(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}
