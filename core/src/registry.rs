use crate::document;
use crate::path::DocPath;
use crate::widget::{Widget, WidgetFactory};
pub use empty_site::EmptySite;

pub struct Registry {
    factories: Vec<&'static dyn document::Factory>,
}

impl Registry {
    fn new() -> Self {
        Self {
            factories: vec![&empty_site::EmptySiteFactory],
        }
    }

    fn resolve(&self, doc_path: &DocPath) -> Box<dyn Widget> {
        for factory in self.factories.iter().rev() {
            if factory.can_handle(manifest) {
                return Ok(factory.create(page_path, doc_path, manifest, context));
            }
        }
    }
}
