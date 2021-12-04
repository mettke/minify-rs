#[derive(Debug, Copy, Clone)]
pub struct HtmlMinifier {
    pub begin: bool,
    pub keep_removing: u8,
    pub last_was_tag_start: bool,
    pub last_was_tag_end: bool,
    pub is_comment: bool,
}

impl Default for HtmlMinifier {
    fn default() -> Self {
        Self::new()
    }
}

impl HtmlMinifier {
    pub const fn new() -> Self {
        Self {
            begin: true,
            keep_removing: 0,
            last_was_tag_start: false,
            last_was_tag_end: false,
            is_comment: false,
        }
    }
}

#[inline]
pub fn keep_element(
    minifier: &mut HtmlMinifier,
    item1: char,
    item2: Option<char>,
    item3: Option<char>,
    item4: Option<char>,
    item5: Option<char>,
    item6: Option<char>,
) -> bool {
    let remove_element = item1.is_ascii_control()
        || is_comment(minifier, item1, item2, item3, item4, item5, item6)
        || is_whitespace_after_tag(minifier, item1, item2)
        || is_whitespace_before_tag_or_whitespace_or_control(minifier, item1, item2);
    if !remove_element {
        minifier.begin = false;
    }
    !remove_element
}

#[inline]
pub fn is_comment(
    minifier: &mut HtmlMinifier,
    item1: char,
    item2: Option<char>,
    item3: Option<char>,
    item4: Option<char>,
    item5: Option<char>,
    item6: Option<char>,
) -> bool {
    if minifier.keep_removing > 0 {
        minifier.keep_removing -= 1;
        return true;
    }
    if equals_comment_start(item1, item2, item3, item4, item5, item6) {
        minifier.is_comment = true;
    }
    if minifier.is_comment {
        if equals_comment_end(item1, item2, item3) {
            minifier.is_comment = false;
            minifier.keep_removing = 2;
        }
        return true;
    }
    false
}

#[inline]
pub fn equals_comment_start(
    item1: char,
    item2: Option<char>,
    item3: Option<char>,
    item4: Option<char>,
    item5: Option<char>,
    item6: Option<char>,
) -> bool {
    item1.eq(&' ')
        && item2.eq(&Some('<'))
        && item3.eq(&Some('!'))
        && item4.eq(&Some('-'))
        && item5.eq(&Some('-'))
        && item6.ne(&Some('['))
        || item1.eq(&'<')
            && item2.eq(&Some('!'))
            && item3.eq(&Some('-'))
            && item4.eq(&Some('-'))
            && item5.ne(&Some('['))
}

#[inline]
pub fn equals_comment_end(item1: char, item2: Option<char>, item3: Option<char>) -> bool {
    item1.eq(&'-') && item2.eq(&Some('-')) && item3.eq(&Some('>'))
}

#[inline]
pub fn is_whitespace_before_tag_or_whitespace_or_control(
    minifier: &mut HtmlMinifier,
    item1: char,
    item2: Option<char>,
) -> bool {
    if item1.is_whitespace() {
        return match item2 {
            Some('<') if minifier.begin => true,
            Some('>' | ' ') => true,
            Some(item) if item.is_ascii_control() => true,
            _ => false,
        };
    }
    false
}

#[inline]
pub fn is_whitespace_after_tag(
    minifier: &mut HtmlMinifier,
    item1: char,
    item2: Option<char>,
) -> bool {
    if minifier.last_was_tag_start && item1.is_whitespace() {
        return true;
    }
    if minifier.last_was_tag_end && item1.is_whitespace() {
        match item2 {
            None => return true,
            Some(c) if c.is_whitespace() => return true,
            _ => {}
        }
    }
    minifier.last_was_tag_start = item1.eq(&'<');
    minifier.last_was_tag_end = item1.eq(&'>');
    false
}
