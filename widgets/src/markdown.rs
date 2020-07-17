use seed::{prelude::*, *};
use semka_core::prelude::*;
use std::collections::BTreeSet;

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
                    let deps = div![md!(&text)].fold(|node, children_deps: Vec<BTreeSet<Path>>| {
                        include_path(&node)
                            .into_iter()
                            .chain(children_deps.into_iter().map(|c| c.into_iter()).flatten())
                            .collect()
                    });
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
            Some(text) => div![md!(text)].deep_map(|node| resolve_include(node, dependencies)),
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

fn include_path(node: &Node<WidgetMsg>) -> Option<Path> {
    match node {
        Node::Element(el) => Some(el),
        _ => None,
    }
    .filter(|el| el.tag == Tag::Img)
    .map(|el| el.attrs.vals.get(&At::Src))
    .flatten()
    .map(|at| match at {
        AtValue::Some(src) => Some(src),
        _ => None,
    })
    .flatten()
    .filter(|url| !is_url_absolute(url))
    .map(|url| url.parse::<Path>().ok())
    .flatten()
}

fn resolve_include(node: Node<WidgetMsg>, dependencies: Dependencies) -> Node<WidgetMsg> {
    if let Some(doc_path) = include_path(&node) {
        dependencies.view(&doc_path)
    } else {
        node
    }
}
