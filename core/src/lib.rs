// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

#[macro_use]
extern crate derivative;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod app;
mod builtin_widgets;
pub mod constants;
pub mod context;
pub mod error;
pub mod manifests;
pub mod node_ext;
pub mod path;
pub mod utils;
pub mod widget;

pub mod prelude {
    pub use super::app::Launcher;
    pub use super::constants::*;
    pub use super::context::Context;
    pub use super::error::*;
    pub use super::manifests::*;
    pub use super::node_ext::NodeExt;
    pub use super::path::Path;
    pub use super::utils::*;
    pub use super::widget::*;
    pub use failure::Error;
}
