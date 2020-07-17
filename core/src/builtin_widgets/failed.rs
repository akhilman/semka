use crate::context::Context;
use crate::path::Path;
use crate::widget::{Dependencies, Widget, WidgetMsg};
use failure::AsFail;
use seed::{prelude::*, *};

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
            h2!["Error"],
            div![format!("Document: {}", self.doc_path)],
            pre![&self.error]
        ]
    }
}
