pub struct JsonMinifier {
    pub is_string: bool,
    pub escaped_quotation: u8,
}

impl Default for JsonMinifier {
    fn default() -> Self {
        Self::new()
    }
}

impl JsonMinifier {
    pub fn new() -> Self {
        JsonMinifier {
            is_string: false,
            escaped_quotation: 0,
        }
    }
}
