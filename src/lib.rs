//! Minification tool for html and json
//!
//! # Usage
//!
//! First add the library to the dependencies of your project like this:
//!
//! ```toml
//! [dependencies]
//! minify = "1.0"
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

// enable additional rustc warnings
#![warn(
    anonymous_parameters, missing_debug_implementations, missing_docs, trivial_casts,
    trivial_numeric_casts, unsafe_code, unstable_features, unused_extern_crates,
    unused_import_braces, unused_qualifications, variant_size_differences
)]
// enable additional clippy warnings
#![cfg_attr(feature = "cargo-clippy", warn(int_plus_one))]
#![cfg_attr(feature = "cargo-clippy", warn(shadow_reuse, shadow_same, shadow_unrelated))]
#![cfg_attr(feature = "cargo-clippy", warn(mut_mut))]
#![cfg_attr(feature = "cargo-clippy", warn(nonminimal_bool))]
#![cfg_attr(feature = "cargo-clippy", warn(pub_enum_variant_names))]
#![cfg_attr(feature = "cargo-clippy", warn(range_plus_one))]
#![cfg_attr(feature = "cargo-clippy", warn(string_add, string_add_assign))]
#![cfg_attr(feature = "cargo-clippy", warn(stutter))]
#![cfg_attr(feature = "cargo-clippy", warn(result_unwrap_used))]

extern crate core;

/// Minification for html content
pub mod html;
/// Minifigation for json content
pub mod json;
