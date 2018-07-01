use io::multi_filter::MultiFilter;
use io::reader::InternalReader;
use json::json_minifier::*;
use std::io::Read;
use std::iter::Iterator;
use std::str::Chars;

mod json_minifier;

type JsonMethod =
    fn(&mut JsonMinifier, &char, Option<&char>, Option<&char>, Option<&char>, Option<&char>)
        -> bool;
type JsonFilter<'a> = MultiFilter<Chars<'a>, JsonMethod, JsonMinifier>;

/// Reader Implementation for JSON minification
pub type Reader<R> = InternalReader<R, JsonMethod, JsonMinifier>;

/// Minifies a given String by JSON minification rules
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
    JsonFilter::new(filtered, keep_element).collect()
}

/// Minifies a given Read by JSON minification rules
///
/// # Example
///
/// ```rust
/// extern crate minify;
/// use std::fs::File;
/// use std::io::Read;
/// use minify::json::minify_from_read;
///
/// fn main() {
///     let mut html_minified = String::new();
///     let mut file = File::open("tests/files/test.json").expect("file not found");
///     minify_from_read(file).read_to_string(&mut html_minified);
/// }
/// ```
#[inline]
pub fn minify_from_read<R: Read>(json: R) -> Reader<R> {
    Reader::new(json, keep_element)
}

#[test]
fn removal_from_read() {
    use std::fs::File;

    let file = File::open("tests/files/test.json").expect("file not found");
    let expected: String = "{\"test\":\"\\\" test2\",\"test2\":\"\",\"test3\":\" \"}".into();
    let mut actual = String::new();
    minify_from_read(file)
        .read_to_string(&mut actual)
        .expect("error at read");
    assert_eq!(actual, expected);
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
