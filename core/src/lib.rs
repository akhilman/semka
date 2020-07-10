// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

pub mod app;
pub mod context;
pub mod error;
pub mod path;
// mod registry;
// mod widget;

use seed::prelude::wasm_bindgen;

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    seed::App::start("app", app::init, app::update, app::view);
}
