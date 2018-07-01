pub struct HtmlMinifier {
    pub keep_removing: u8,
    pub last_was_tag_start_or_end: bool,
    pub is_comment: bool,
}

impl Default for HtmlMinifier {
    fn default() -> Self {
        Self::new()
    }
}

impl HtmlMinifier {
    pub fn new() -> Self {
        HtmlMinifier {
            keep_removing: 0,
            last_was_tag_start_or_end: true,
            is_comment: false,
        }
    }
}
