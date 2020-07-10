use crate::error::WidgetError;
use crate::path::{DocPath, PagePath};
use crate::widget::{DocManifest, Widget, WidgetFactory};

pub struct Register {
    factories: Vec<Box<dyn WidgetFactory>>,
}

impl Register {
    pub fn new() -> Self {
        Self { factories: vec![] }
    }
    pub fn add_widget_factory<F>(mut self, factory: F) -> Self
    where
        F: WidgetFactory + 'static,
    {
        self.factories.push(Box::new(factory));
        self
    }

    pub fn resolve(
        &self,
        manifest: DocManifest,
        doc_path: &DocPath,
        page_path: &PagePath,
    ) -> Result<Box<dyn Widget>, WidgetError> {
        for factory in self.factories.iter().rev() {
            if factory.can_handle(&manifest) {
                return factory.create(manifest, doc_path, page_path);
            }
        }
        Err(WidgetError::new(&manifest.widget, "Can not find widget"))
    }
}
