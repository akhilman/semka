use crate::constants::MAX_WIDGET_RECURSION;
use crate::context::Context;
use crate::path::Path;
use crate::widget::{Widget, WidgetMsg};
use seed::{prelude::*, *};
use std::collections::BTreeMap;

#[derive(Clone, Copy)]
pub struct Dependencies<'a> {
    widgets: &'a BTreeMap<Path, Box<dyn Widget>>,
    recursion_level: usize,
    ctx: &'a Context,
}

impl<'a> Dependencies<'a> {
    pub(crate) fn new(widgets: &'a BTreeMap<Path, Box<dyn Widget>>, ctx: &'a Context) -> Self {
        Self {
            widgets,
            recursion_level: 0,
            ctx,
        }
    }

    fn dig_in(&self) -> Self {
        Self {
            recursion_level: self.recursion_level + 1,
            widgets: self.widgets,
            ctx: self.ctx,
        }
    }

    pub fn view(&self, path: &Path) -> Node<WidgetMsg> {
        if self.recursion_level <= MAX_WIDGET_RECURSION {
            if let Some(widget) = self.widgets.get(path) {
                widget.view(self.dig_in(), self.ctx)
            } else {
                custom![
                    Tag::from("failed-include"),
                    attrs! {
                        At::from("doc") => path.to_string(),
                        At::from("reason") => "not found",
                    }
                ]
            }
        } else {
            custom![
                Tag::from("failed-include"),
                attrs! {
                    At::from("doc") => path.to_string(),
                    At::from("reason") => "recursion level exceeded",
                }
            ]
        }
    }
}
