# minify

Minification tool for html and json

## Usage

First add the library to the dependencies of your project like this:

```toml
[dependencies]
minify = "1.0"
```

Afterwards you can import the library like this:

```rust
extern crate minify;
```

## Minify Html

The following rules are applied for html minification:

* Removal of ascii control characters
* Removal of comments
* Removal of multiple whitespaces
* Removal of whitespaces before and after greater-than and less-than signs
  * `_<_html_>_` => `<html>`

```rust
extern crate minify;
use minify::html::minify;

fn main() {
    let html = r#"
        <html>
            <head>
            </head>
            <body>
            </body>
        <html>
    "#.into();
    let html_minified = minify(html);
}
```

## Minify JSON

The following rules are applied for json minification:

* Removal of ascii control characters
* Removal of whitespaces outside of strings

```rust
extern crate minify;
use minify::json::minify;

fn main() {
    let json = r#"
           {
               "test": "test",
               "test2": 2
           }
       "#.into();
    let html_minified = minify(json);
}
```

License: MIT
