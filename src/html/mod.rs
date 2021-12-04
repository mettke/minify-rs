use crate::{
    html::html_minifier::{keep_element, HtmlMinifier},
    io::{multi_filter::MultiFilter, reader::InternalReader},
};
use std::{io::Read, str::Chars};

mod html_minifier;

type HtmlMethod = fn(
    &mut HtmlMinifier,
    char,
    Option<char>,
    Option<char>,
    Option<char>,
    Option<char>,
    Option<char>,
) -> bool;
type HtmlFilter<'a> = MultiFilter<Chars<'a>, HtmlMethod, HtmlMinifier>;

/// Reader Implementation for HTML minification
pub type Reader<R> = InternalReader<R, HtmlMethod, HtmlMinifier>;

/// Minifies a given String by HTML minification rules
///
/// # Example
///
/// ```rust
/// extern crate minify;
/// use minify::html::minify;
///
/// fn main() {
///     let html = r#"
///         <html>
///             <head>
///             </head>
///             <body>
///             </body>
///         <html>
///     "#;
///     let html_minified = minify(html);
/// }
/// ```
#[inline]
#[must_use]
pub fn minify(html: &str) -> String {
    let filtered = html.chars();
    HtmlFilter::new(filtered, keep_element).collect()
}

/// Minifies a given Read by HTML minification rules
///
/// # Example
///
/// ```rust
/// extern crate minify;
/// use std::fs::File;
/// use std::io::Read;
/// use minify::html::minify_from_read;
///
/// fn main() {
///     let mut html_minified = String::new();
///     let mut file = File::open("tests/files/test.html").expect("file not found");
///     minify_from_read(file).read_to_string(&mut html_minified);
/// }
/// ```
#[inline]
pub fn minify_from_read<R: Read>(html: R) -> Reader<R> {
    Reader::new(html, keep_element)
}

#[test]
fn removal_from_read() {
    use std::fs::File;

    let file = File::open("tests/files/test.html").expect("file not found");
    let expected = "<html><head> <!--[if lte IE 8]> Important comment test \
         <![endif]--></head><body></body><html>";
    let mut actual = String::new();
    let _ = minify_from_read(file)
        .read_to_string(&mut actual)
        .expect("error at read");
    assert_eq!(actual, expected);
}

#[test]
fn removal_of_control_characters() {
    let input = "\n";
    let expected = "";
    let actual = minify(input);
    assert_eq!(actual, expected);
}

#[test]
fn removal_of_whitespace_outside_of_tags() {
    let input = r#"
            <html>
                <head>
                </head>
                <body>
                </body>
            <html>
        "#;
    let expected = "<html> <head> </head> <body> </body> <html>";
    let actual = minify(input);
    assert_eq!(actual, expected);
}

#[test]
fn removal_of_whitespace_inside_of_tags() {
    let input = r#"
            <html>
                < head >
                </head>
                <body>
                </body>
            <html>
        "#;
    let expected = "<html> <head> </head> <body> </body> <html>";
    let actual = minify(input);
    assert_eq!(actual, expected);
}

#[test]
fn removal_of_comments_outside_of_tags() {
    let input = r#"
            <html>
                <!-- comment data -->
                <!--
                multi line comment
                -->
                <head>
                </head>
                <body>
                </body>
            <html>
        "#;
    let expected = "<html> <head> </head> <body> </body> <html>";
    let actual = minify(input);
    assert_eq!(actual, expected);
}

#[test]
fn removal_of_comments_inside_of_tags() {
    let input = r#"
            <html>
                <head <!-- comment data -->
                <!--
                multi line comment
                -->>
                </head>
                <body>
                </body>
            <html>
        "#;
    let expected = "<html> <head> </head> <body> </body> <html>";
    let actual = minify(input);
    assert_eq!(actual, expected);
}

#[test]
fn removal_of_double_whitespace_outside_of_tags() {
    let input = r#"
            <html>  test
                <head>
                </head>
                <body>
                </body>
            <html>
        "#;
    let expected = "<html> test <head> </head> <body> </body> <html>";
    let actual = minify(input);
    assert_eq!(actual, expected);
}

#[test]
fn keep_whitespace_between_content_but_remove_double() {
    let input = r#"
            <html>  test  settings data
                <head>
                </head>
                <body>
                </body>
            <html>
        "#;
    let expected = "<html> test settings data <head> </head> <body> </body> <html>";
    let actual = minify(input);
    assert_eq!(actual, expected);
}

#[test]
fn keep_important_comments() {
    let input = r#"
            <html>
                <head>
                <!--[if lte IE 8]>
                Important comment test
                <![endif]-->
                </head>
                <body>
                </body>
            <html>
        "#;
    let expected = "<html> <head> <!--[if lte IE 8]> Important comment test \
         <![endif]--> </head> <body> </body> <html>";
    let actual = minify(input);
    assert_eq!(actual, expected);
}

#[test]
fn keep_important_whitespaces() {
    let input = r#"<p>Foo <span>Bar</span> <span>Baz</span></p>"#;
    let expected = "<p>Foo <span>Bar</span> <span>Baz</span></p>";
    let actual = minify(input);
    assert_eq!(actual, expected);
}

#[test]
fn keep_pre() {
    let input = r#"<pre>
    should respect this          does it?
</pre>"#;
    let expected = r#"<pre>
    should respect this          does it?
</pre>"#;
    let actual = minify(input);
    assert_eq!(actual, expected);
}
