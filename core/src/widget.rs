use crate::context::Context;
use crate::error::WidgetError;
use crate::manifests::DocManifest;
use crate::path::Path;
use failure::Error;
use std::any::Any;

pub mod orders;

pub use orders::{WidgetCmd, WidgetOrders};

pub trait Widget: std::fmt::Debug {
    fn init(&mut self, _ctx: &Context) -> Result<Option<WidgetOrders>, Error> {
        Ok(None)
    }
    fn update(&mut self, _msg: &WidgetMsg, _ctx: &Context) -> Result<Option<WidgetOrders>, Error> {
        Ok(None)
    }
    fn view(&self, ctx: &Context) -> seed::virtual_dom::Node<WidgetMsg>;
}

pub enum WidgetMsg {
    CmdResult(Box<dyn Any>),
}

pub trait WidgetFactory: std::fmt::Debug {
    fn can_handle(&self) -> &'static [&'static str];
    fn create(&self, doc_path: Path, manifest: DocManifest)
        -> Result<Box<dyn Widget>, WidgetError>;
}
