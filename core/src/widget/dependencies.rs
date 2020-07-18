use crate::constants::MAX_WIDGET_RECURSION;
use crate::context::Context;
use crate::path::Path;
use crate::widget::{Widget, WidgetMsg};
use failure::{format_err, AsFail};
use seed::prelude::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy)]
pub struct Dependencies<'a> {
    doc_path: Option<&'a Path>,
    widgets: &'a BTreeMap<Path, Box<dyn Widget>>,
    dependencies: &'a BTreeMap<Path, BTreeSet<Path>>,
    recursion_level: usize,
    ctx: &'a Context,
}

impl<'a> Dependencies<'a> {
    pub(crate) fn new(
        widgets: &'a BTreeMap<Path, Box<dyn Widget>>,
        dependencies: &'a BTreeMap<Path, BTreeSet<Path>>,
        ctx: &'a Context,
    ) -> Self {
        Self {
            doc_path: None,
            widgets,
            dependencies,
            recursion_level: 0,
            ctx,
        }
    }

    fn dig_in(&self, doc_path: &'a Path) -> Self {
        Self {
            doc_path: Some(doc_path),
            recursion_level: self.recursion_level + 1,
            widgets: self.widgets,
            dependencies: self.dependencies,
            ctx: self.ctx,
        }
    }

    pub fn view(&self, path: &'a Path) -> Node<WidgetMsg> {
        let mut node = if self.recursion_level > MAX_WIDGET_RECURSION {
            Err(format_err!("Recursion level exceeded"))
        } else if self
            .doc_path
            .map(|self_path| {
                !self
                    .dependencies
                    .get(self_path)
                    .map(|dep| dep.contains(path))
                    .unwrap_or(false)
            })
            .unwrap_or(false)
        {
            Err(format_err!(
                "Document \"{}\" not requested by \"{}\"",
                path,
                self.doc_path.unwrap_or(&Path::new_absolute())
            ))
        } else if let Some(widget) = self.widgets.get(path) {
            let mut node = widget.view(self.dig_in(path), self.ctx);
            node.add_attr("data-widget-name", widget.widget_name());
            node.add_class("widget");
            for cls in widget.classes() {
                node.add_class(std::borrow::Cow::from(*cls));
            }
            Ok(node)
        } else {
            Err(format_err!("Required document \"{}\" not loaded", path))
        }
        .unwrap_or_else(|err| {
            failed_widget(self.doc_path.cloned().unwrap_or(Path::new_absolute()), &err)
                .view(self.dig_in(path), self.ctx)
        });
        node.add_attr("data-doc-path", path.to_string());
        node
    }
}

fn failed_widget(doc_path: Path, error: &impl AsFail) -> Box<dyn Widget> {
    crate::builtin_widgets::Failed::new(doc_path, error)
}
