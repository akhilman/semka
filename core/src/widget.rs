use crate::context::Context;
use crate::error::WidgetError;
use crate::manifests::DocManifest;
use crate::path::Path;
use std::any::Any;
use std::collections::BTreeSet;

pub mod orders;

pub use orders::{WidgetCmd, WidgetOrders};

pub trait Widget: std::fmt::Debug {
    fn dependencies(&self) -> BTreeSet<Path> {
        BTreeSet::new()
    }
    fn update(&mut self, _msg: &WidgetMsg, _context: &mut WidgetOrders, _ctx: &Context) {}
    fn view(&self, ctx: &Context) -> seed::virtual_dom::Node<WidgetMsg>;
}

pub enum WidgetMsg {
    CmdResult(Box<dyn Any>),
}

pub trait WidgetFactory: std::fmt::Debug {
    fn can_handle(&self, manifest: &DocManifest) -> bool;
    fn create(&self, doc_path: Path, manifest: DocManifest)
        -> Result<Box<dyn Widget>, WidgetError>;
}
