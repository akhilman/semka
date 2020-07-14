use crate::context::Context;
use seed::{prelude::*, *};

// ------ ------
//     View
// ------ ------

pub fn view<Ms>(_ctx: &Context) -> Node<Ms> {
    div! {"About"}
}
