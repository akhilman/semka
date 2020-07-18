use crate::context::Context;
use crate::manifests::SiteManifest;
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

pub fn init(url: Url, orders: &mut impl Orders<Msg>, ctx: &Context) -> Model {
    Model {}
}

// ------ ------
//     Model
// ------ ------

#[derive(Debug)]
pub struct Model {}

// ------ ------
//    Update
// ------ ------

#[derive(Debug)]
pub enum Msg {
    UrlChanged(Url),
    SiteManifestChanged(SiteManifest),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, ctx: &Context) {}

// ------ ------
//     View
// ------ ------

pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
    div! {"Not implemented"}
}
