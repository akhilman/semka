// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

mod markdown;
mod stylesheet;

pub mod widgets {
    pub use super::markdown::MarkdownFactory;
    pub use super::stylesheet::StylesheetFactory;
}
