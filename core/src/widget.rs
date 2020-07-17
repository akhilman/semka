use crate::context::Context;
use crate::error::{FetchError, WidgetError};
use crate::manifests::DocManifest;
use crate::path::Path;
use bytes::Bytes;
use failure::Error;
use std::any::Any;

mod dependencies;
mod orders;

pub use dependencies::Dependencies;
pub use orders::{WidgetCmd, WidgetOrders};

pub trait Widget: std::fmt::Debug {
    fn init(&mut self, _path: &Path, _ctx: &Context) -> Result<Option<WidgetOrders>, Error> {
        Ok(None)
    }
    fn update(&mut self, _msg: WidgetMsg, _ctx: &Context) -> Result<Option<WidgetOrders>, Error> {
        Ok(None)
    }
    fn view<'a>(
        &'a self,
        dependencies: Dependencies<'a>,
        ctx: &'a Context,
    ) -> seed::virtual_dom::Node<WidgetMsg>;
}

pub enum WidgetMsg {
    CmdResult(Box<dyn Any>),
    FetchBytesResult(Path, Result<Bytes, FetchError>),
    FetchJsonResult(Path, Result<serde_json::Value, FetchError>),
    FetchTextResult(Path, Result<String, FetchError>),
}

pub trait WidgetFactory: std::fmt::Debug {
    fn can_handle(&self) -> &'static [&'static str];
    fn create(&self, doc_path: Path, manifest: DocManifest)
        -> Result<Box<dyn Widget>, WidgetError>;
}
