use crate::error::WidgetError;
use crate::widget::WidgetFactory;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Registry {
    factories: Vec<Box<dyn WidgetFactory>>,
    factory_by_widget: BTreeMap<&'static str, usize>,
}

impl Registry {
    pub(crate) fn new() -> Self {
        Self {
            factories: vec![],
            factory_by_widget: BTreeMap::new(),
        }
    }
    pub(crate) fn add_widget<F>(mut self, factory: F) -> Self
    where
        F: WidgetFactory + 'static,
    {
        let index = self.factories.len();
        factory.can_handle().iter().for_each(|widget| {
            self.factory_by_widget.insert(widget, index);
        });
        self.factories.push(Box::new(factory));
        self
    }

    pub fn get_widget(&self, widget: &str) -> Result<&dyn WidgetFactory, WidgetError> {
        let index = self
            .factory_by_widget
            .get(widget)
            .copied()
            .ok_or(WidgetError::new(widget, "No such widget"))?;
        let factory = self
            .factories
            .get(index)
            .ok_or(WidgetError::new(widget, "Unexpected factory index"))?;
        Ok(factory.as_ref())
    }
}
