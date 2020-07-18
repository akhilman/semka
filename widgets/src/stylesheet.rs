use seed::{prelude::*, *};
use semka_core::prelude::*;

const WIDGET_NAME: &'static str = "semka-0.1-stylesheet";
const WIDGET_CLASSES: &'static [&'static str] = &[WIDGET_NAME, "stylesheet"];
const CAN_HANDLE: &'static [&'static str] = &[WIDGET_NAME];
const CSS_FILE: &str = "style.css";

#[derive(Debug)]
pub struct Stylesheet {
    doc_path: Path,
}

impl Stylesheet {
    pub fn new() -> Box<dyn Widget> {
        Box::new(Self {
            doc_path: Path::new(),
        })
    }
}

impl Widget for Stylesheet {
    fn init(&mut self, doc_path: &Path, _ctx: &Context) -> Result<Option<WidgetOrders>, Error> {
        self.doc_path = doc_path.clone();
        Ok(Some(WidgetOrders::new().update_deps(
            vec![self.doc_path.tail()].into_iter().collect(),
        )))
    }
    fn view(&self, dependencies: Dependencies, _ctx: &Context) -> Node<WidgetMsg> {
        assert!(!self.doc_path.is_empty(), "doc_path is empty");
        div![
            raw!(format!(
                r#"<link rel="stylesheet" href="{}/{}/{}"/>"#,
                DOC_DIR,
                self.doc_path.head(),
                CSS_FILE
            )
            .as_str()),
            dependencies.view(&self.doc_path.tail())
        ]
    }

    fn widget_name(&self) -> &'static str {
        WIDGET_NAME
    }
    fn classes(&self) -> &'static [&'static str] {
        WIDGET_CLASSES
    }
}

#[derive(Debug)]
pub struct StylesheetFactory {}

impl StylesheetFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl WidgetFactory for StylesheetFactory {
    fn can_handle(&self) -> &'static [&'static str] {
        CAN_HANDLE
    }
    fn create(&self, _: Path, _: DocManifest) -> Result<Box<dyn Widget>, WidgetError> {
        Ok(Stylesheet::new())
    }
}
