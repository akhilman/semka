// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::prelude::wasm_bindgen;
use semka_core::prelude::*;
use semka_widgets::widgets;

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    Launcher::new()
        .add_widget(widgets::MarkdownFactory::new())
        .root_element("app")
        .start();
}
