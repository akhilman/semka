use crate::context::Context;
use crate::error::WidgetError;
use crate::manifests::DocManifest;
use crate::path::Path;
use crate::widget::{Widget, WidgetFactory, WidgetMsg};
use seed::{prelude::*, *};

#[derive(Debug)]
pub struct Loading {}

impl Loading {
    pub fn new() -> Box<dyn Widget> {
        Box::new(Self {})
    }
}

impl Widget for Loading {
    fn view(&self, _ctx: &Context) -> Node<WidgetMsg> {
        div!["Loading..."]
    }
}

#[derive(Debug)]
pub struct LoadingFactory {}

impl LoadingFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl WidgetFactory for LoadingFactory {
    fn can_handle(&self) -> &'static [&'static str] {
        const CAN_HANDLE: &'static [&'static str] = &["loading"];
        CAN_HANDLE
    }
    fn create(&self, _: Path, _: DocManifest) -> Result<Box<dyn Widget>, WidgetError> {
        Ok(Loading::new())
    }
}
