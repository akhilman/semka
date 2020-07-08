// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

mod context;
mod error;
// mod document;
mod path;
// mod registry;
mod page;
// mod widget;
// mod utils;

use seed::prelude::wasm_bindgen;
use seed::App;

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", page::init, page::update, page::view);
}
