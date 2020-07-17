use seed::{prelude::*, *};
use semka_core::prelude::*;

const CAN_HANDLE: &'static [&'static str] = &["semka-0.1-stylesheet"];
const CSS_FILE: &str = "style.css";

#[derive(Debug)]
pub struct Stylesheet {
    path_head: Path,
    path_tail: Path,
}

impl Stylesheet {
    pub fn new() -> Box<dyn Widget> {
        Box::new(Self {
            path_head: Path::new(),
            path_tail: Path::new(),
        })
    }
}

impl Widget for Stylesheet {
    fn init(&mut self, doc_path: &Path, _ctx: &Context) -> Result<Option<WidgetOrders>, Error> {
        self.path_head = doc_path.head();
        self.path_tail = doc_path.tail();
        Ok(Some(WidgetOrders::new().update_deps(
            vec![self.path_tail.clone()].into_iter().collect(),
        )))
    }
    fn view(&self, dependencies: Dependencies, _ctx: &Context) -> Node<WidgetMsg> {
        section![
            C!["widget", "stylesheet", "semka-0.1-stylesheet"],
            raw!(format!(
                r#"<link rel="stylesheet" href="{}/{}/{}"/>"#,
                DOC_DIR, self.path_head, CSS_FILE
            )
            .as_str()),
            dependencies.view(&self.path_tail)
        ]
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
