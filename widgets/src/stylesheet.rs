use seed::{prelude::*, *};
use semka_core::prelude::*;

const CAN_HANDLE: &'static [&'static str] = &["semka-0.1-stylesheet"];
const CSS_FILE: &str = "style.css";

#[derive(Debug)]
pub struct Stylesheet {
    path_tail: Path,
    stylesheet: Option<String>,
}

impl Stylesheet {
    pub fn new() -> Box<dyn Widget> {
        Box::new(Self {
            path_tail: Path::new(),
            stylesheet: None,
        })
    }
}

impl Widget for Stylesheet {
    fn init(&mut self, doc_path: &Path, _ctx: &Context) -> Result<Option<WidgetOrders>, Error> {
        self.path_tail = doc_path.tail();
        Ok(Some(
            WidgetOrders::new()
                .fetch_text(CSS_FILE.parse()?)
                .update_deps(vec![self.path_tail.clone()].into_iter().collect()),
        ))
    }
    fn update(&mut self, msg: WidgetMsg, _ctx: &Context) -> Result<Option<WidgetOrders>, Error> {
        match msg {
            WidgetMsg::FetchTextResult(_fpath, Ok(style)) => {
                self.stylesheet.replace(style);
                Ok(None)
            }
            WidgetMsg::FetchTextResult(_fpath, Err(err)) => Err(err.into()),
            _ => Ok(None),
        }
    }
    fn view(&self, dependencies: Dependencies, _ctx: &Context) -> Node<WidgetMsg> {
        match &self.stylesheet {
            Some(style) => div![
                raw!(format!("<style>{}</style>", style).as_str()),
                dependencies.view(&self.path_tail)
            ],
            None => dependencies.view(&self.path_tail),
        }
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
