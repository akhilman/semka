use crate::context::Context;
use crate::path::Path;
use crate::widget::{Dependencies, Widget, WidgetMsg};
use failure::AsFail;
use seed::{prelude::*, *};

const WIDGET_NAME: &'static str = "failed";
const WIDGET_CLASSES: &'static [&'static str] = &[WIDGET_NAME];

#[derive(Debug)]
pub struct Failed {
    doc_path: Path,
    error: String,
}

impl Failed {
    pub fn new(doc_path: Path, error: &impl AsFail) -> Box<dyn Widget> {
        Box::new(Self {
            doc_path,
            error: error.as_fail().to_string(),
        })
    }
}

impl Widget for Failed {
    fn view(&self, _dependencies: Dependencies, _ctx: &Context) -> Node<WidgetMsg> {
        div![
            C!["failed-widget"],
            attrs! {
                At::Custom("data-doc-path".into()) => self.doc_path.to_string(),
            },
            h2!["Error"],
            div![format!("Document: {}", self.doc_path)],
            pre![&self.error]
        ]
    }

    fn widget_name(&self) -> &'static str {
        WIDGET_NAME
    }
    fn classes(&self) -> &'static [&'static str] {
        WIDGET_CLASSES
    }
}
