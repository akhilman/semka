use crate::context::Context;
use crate::error::WidgetError;
use crate::manifests::DocManifest;
use crate::path::Path;
use crate::utils;
use crate::widget::{Dependencies, Widget, WidgetFactory, WidgetMsg};
use seed::{prelude::*, *};

const WIDGET_NAME: &'static str = "semka-0.1-loading";
const WIDGET_CLASSES: &'static [&'static str] = &[WIDGET_NAME, "loading"];

#[derive(Debug)]
pub struct Loading {}

impl Loading {
    pub fn new() -> Box<dyn Widget> {
        Box::new(Self {})
    }
}

impl Widget for Loading {
    fn view(&self, _dependencies: Dependencies, _ctx: &Context) -> Node<WidgetMsg> {
        div![utils::show_spinner()]
    }

    fn widget_name(&self) -> &'static str {
        WIDGET_NAME
    }
    fn classes(&self) -> &'static [&'static str] {
        WIDGET_CLASSES
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
