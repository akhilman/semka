use crate::utils;
use seed::{prelude::*, *};
use semka_core::prelude::*;

const CAN_HANDLE: &'static [&'static str] = &["semka-0.1-markdown"];
const TEXT_FILE: &str = "text.md";

#[derive(Debug)]
pub struct Markdown {
    path_tail: Path,
    path_head: Path,
    text: Option<String>,
}

impl Markdown {
    pub fn new() -> Box<dyn Widget> {
        Box::new(Self {
            path_head: Path::new(),
            path_tail: Path::new(),
            text: None,
        })
    }
}

impl Widget for Markdown {
    fn init(&mut self, doc_path: &Path, _ctx: &Context) -> Result<Option<WidgetOrders>, Error> {
        self.path_head = doc_path.head();
        self.path_tail = doc_path.tail();
        Ok(Some(WidgetOrders::new().fetch_text(TEXT_FILE.parse()?)))
    }
    fn update(&mut self, msg: WidgetMsg, _ctx: &Context) -> Result<Option<WidgetOrders>, Error> {
        match msg {
            WidgetMsg::FetchTextResult(_fpath, result) => match result {
                Ok(text) => {
                    let deps = utils::find_include_deps(div![md!(&text)]);
                    self.text.replace(text);
                    Ok(Some(WidgetOrders::new().update_deps(deps)))
                }
                Err(err) => Err(err.into()),
            },
            _ => Ok(None),
        }
    }
    fn view(&self, dependencies: Dependencies, _ctx: &Context) -> Node<WidgetMsg> {
        match &self.text {
            Some(text) => {
                div![md!(text)].deep_map(|node| utils::resolve_include(node, dependencies))
            }
            None => show_spinner(),
        }
    }
}

#[derive(Debug)]
pub struct MarkdownFactory {}

impl MarkdownFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl WidgetFactory for MarkdownFactory {
    fn can_handle(&self) -> &'static [&'static str] {
        CAN_HANDLE
    }
    fn create(&self, _: Path, _: DocManifest) -> Result<Box<dyn Widget>, WidgetError> {
        Ok(Markdown::new())
    }
}
