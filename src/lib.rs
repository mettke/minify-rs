//! Minification tool for html and json
//!
//! # Usage
//!
//! First add the library to the dependencies of your project like this:
//!
//! ```toml
//! [dependencies]
//! minify = "1.2"
//! ```
//!
//! Afterwards you can import the library like this:
//!
//! ```rust
//! extern crate minify;
//! ```
//!
//! # Minify Html
//!
//! The following rules are applied for html minification:
//!
//! * Removal of ascii control characters
//! * Removal of comments
//! * Removal of multiple whitespaces
//! * Removal of whitespaces before and after greater-than and less-than signs
//!   * `_<_html_>_` => `<html>`
//!
//! ```rust
//! extern crate minify;
//! use minify::html::minify;
//!
//! fn main() {
//!     let html = r#"
//!         <html>
//!             <head>
//!             </head>
//!             <body>
//!             </body>
//!         <html>
//!     "#.into();
//!     let html_minified = minify(html);
//! }
//! ```
//!
//! # Minify JSON
//!
//! The following rules are applied for json minification:
//!
//! * Removal of ascii control characters
//! * Removal of whitespaces outside of strings
//!
//! ```rust
//! extern crate minify;
//! use minify::json::minify;
//!
//! fn main() {
//!     let json = r#"
//!            {
//!                "test": "test",
//!                "test2": 2
//!            }
//!        "#.into();
//!     let json_minified = minify(json);
//! }
//! ```

#![warn(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    box_pointers,
    confusable_idents,
    deprecated_in_future,
    // elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    indirect_structural_match,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_debug_implementations,
    missing_docs,
    missing_doc_code_examples,
    non_ascii_idents,
    private_doc_tests,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unaligned_references,
    // unreachable_pub,
    unsafe_code,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#![warn(
    clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style
)]
#![allow(
    clippy::implicit_return,
    clippy::shadow_unrelated,
    clippy::struct_excessive_bools,
    clippy::module_name_repetitions,
    clippy::match_wildcard_for_single_variants
)]

/// Minification for html content
pub mod html;
mod io;
/// Minifigation for json content
pub mod json;
