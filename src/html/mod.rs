use html::html_minifier::HtmlMinifier;
use html::multi_filter::MultiFilter;

mod html_minifier;
mod multi_filter;

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
///     "#.into();
///     let html_minified = minify(html);
/// }
/// ```
#[inline]
pub fn minify(html: &str) -> String {
    let filtered = html.chars();
    MultiFilter::new(filtered, keep_element).collect()
}

#[inline]
fn keep_element(
    minifier: &mut HtmlMinifier,
    item1: &char,
    item2: Option<&char>,
    item3: Option<&char>,
    item4: Option<&char>,
    item5: Option<&char>,
) -> bool {
    let remove_element = item1.is_ascii_control()
        || is_comment(minifier, item1, item2, item3, item4, item5)
        || is_whitespace_after_tag(minifier, item1)
        || is_whitespace_before_tag_or_whitespace_or_control(item1, item2);
    !remove_element
}

#[inline]
fn is_comment(
    minifier: &mut HtmlMinifier,
    item1: &char,
    item2: Option<&char>,
    item3: Option<&char>,
    item4: Option<&char>,
    item5: Option<&char>,
) -> bool {
    if minifier.keep_removing > 0 {
        minifier.keep_removing -= 1;
        return true;
    }
    if equals_comment_start(item1, item2, item3, item4, item5) {
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
fn equals_comment_start(
    item1: &char,
    item2: Option<&char>,
    item3: Option<&char>,
    item4: Option<&char>,
    item5: Option<&char>,
) -> bool {
    item1.eq(&'<')
        && item2.eq(&Some(&'!'))
        && item3.eq(&Some(&'-'))
        && item4.eq(&Some(&'-'))
        && item5.ne(&Some(&'['))
}

#[inline]
fn equals_comment_end(item1: &char, item2: Option<&char>, item3: Option<&char>) -> bool {
    item1.eq(&'-') && item2.eq(&Some(&'-')) && item3.eq(&Some(&'>'))
}

#[inline]
fn is_whitespace_before_tag_or_whitespace_or_control(item1: &char, item2: Option<&char>) -> bool {
    if item1.is_whitespace() {
        return match item2 {
            Some(&'<') => true,
            Some(&'>') => true,
            Some(&' ') => true,
            Some(item) if item.is_ascii_control() => true,
            _ => false,
        };
    }
    false
}

#[inline]
fn is_whitespace_after_tag(minifier: &mut HtmlMinifier, item1: &char) -> bool {
    if minifier.last_was_tag_start_or_end && item1.is_whitespace() {
        return true;
    }
    minifier.last_was_tag_start_or_end = item1.eq(&'<') || item1.eq(&'>');
    false
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
            <html>
                <head>
                </head>
                <body>
                </body>
            <html>
        "#.into();
    let expected: String = "<html><head></head><body></body><html>".into();
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
        "#.into();
    let expected: String = "<html><head></head><body></body><html>".into();
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
        "#.into();
    let expected: String = "<html><head></head><body></body><html>".into();
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
        "#.into();
    let expected: String = "<html><head></head><body></body><html>".into();
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
        "#.into();
    let expected: String = "<html>test<head></head><body></body><html>".into();
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
        "#.into();
    let expected: String = "<html>test settings data<head></head><body></body><html>".into();
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
        "#.into();
    let expected: String = String::from(
        "<html><head><!--[if lte IE 8]>Important comment test\
         <![endif]--></head><body></body><html>",
    );
    let actual = minify(input);
    assert_eq!(actual, expected);
}
