use crate::error::WidgetError;
use crate::manifests::DocManifest;
use crate::path::Path;
use crate::widget::{Widget, WidgetFactory};

#[derive(Debug)]
pub struct Registry {
    factories: Vec<Box<dyn WidgetFactory>>,
}

impl Registry {
    pub fn new() -> Self {
        Self { factories: vec![] }
    }
    pub fn add_widget<F>(mut self, factory: F) -> Self
    where
        F: WidgetFactory + 'static,
    {
        self.factories.push(Box::new(factory));
        self
    }

    pub fn resolve(&self, manifest: &DocManifest) -> Result<&dyn WidgetFactory, WidgetError> {
        for factory in self.factories.iter().rev() {
            if factory.can_handle(&manifest) {
                return Ok(factory.as_ref());
            }
        }
        Err(WidgetError::new(&manifest.widget, "Can not find widget"))
    }
}
