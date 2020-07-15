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
    fn update(&mut self, msg: &WidgetMsg, context: &mut WidgetOrders, ctx: &Context);
    fn view(&self, context: &mut WidgetOrders, ctx: &Context);
}

pub enum WidgetMsg {
    CmdResult(Box<dyn Any>),
}

pub trait WidgetFactory: std::fmt::Debug {
    fn can_handle(&self, manifest: &DocManifest) -> bool;
    fn create(&self, manifest: DocManifest, doc_path: Path)
        -> Result<Box<dyn Widget>, WidgetError>;
}
