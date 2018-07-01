use json::json_minifier::JsonMinifier;
use json::multi_filter::MultiFilter;

mod json_minifier;
mod multi_filter;

/// Minifies a given String by HTML minification rules
///
/// # Example
///
/// ```rust
/// extern crate minify;
/// use minify::json::minify;
///
/// fn main() {
///     let json = r#"
///            {
///                "test": "test",
///                "test2": 2
///            }
///        "#.into();
///     let json_minified = minify(json);
/// }
/// ```
#[inline]
pub fn minify(json: &str) -> String {
    let filtered = json.chars();
    MultiFilter::new(filtered, keep_element).collect()
}

#[inline]
fn keep_element(minifier: &mut JsonMinifier, item1: &char, item2: Option<&char>) -> bool {
    let remove_element =
        item1.is_ascii_control() || is_whitespace_outside_string(minifier, item1, item2);
    !remove_element
}

#[inline]
fn is_whitespace_outside_string(
    minifier: &mut JsonMinifier,
    item1: &char,
    item2: Option<&char>,
) -> bool {
    if !minifier.is_string && item1.eq(&'"') {
        minifier.is_string = true;
    } else if minifier.is_string {
        if item1.eq(&'\\') && item2.eq(&Some(&'"')) {
            minifier.escaped_quotation = 4;
        }
        if minifier.escaped_quotation > 0 {
            minifier.escaped_quotation -= 1;
        } else if item1.eq(&'"') {
            minifier.is_string = false;
        }
    }
    !minifier.is_string && item1.is_whitespace()
}

#[test]
fn removal_of_control_characters() {
    let input = "\n".into();
    let expected: String = "".into();
    let actual = minify(input);
    assert_eq!(actual, expected);
}

#[test]
fn removal_of_whitespace_outside_of_tags() {
    let input = r#"
            {
              "test": "\" test2",
              "test2": "",
              "test3": " "
            }
        "#.into();
    let expected: String = "{\"test\":\"\\\" test2\",\"test2\":\"\",\"test3\":\" \"}".into();
    let actual = minify(input);
    assert_eq!(actual, expected);
}
