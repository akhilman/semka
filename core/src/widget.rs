use crate::context::Context;
use crate::error::SemkaError;
use crate::path::{DocPath, FilePath, PagePath};
use seed::browser::fetch;
use serde::{Deserialize, Serialize};
use serde_json;
use std::any::Any;
use std::collections::BTreeSet;

pub trait Widget {
    fn dependencies(&self) -> BTreeSet<DocPath> {
        BTreeSet::new()
    }
    fn update(&mut self, msg: &WidgetMsg, context: &mut Context);
    fn view(&self, context: &mut Context);
}

pub struct WidgetMsg {
    pub document: DocPath,
    pub page: PagePath,
    pub message: WidgetMsgContent,
}

pub enum WidgetMsgContent {
    JsonFetched(fetch::Result<serde_json::Value>),
    Custom(Box<dyn Any>),
}

pub trait WidgetFactory {
    fn can_handle(&self, manifest: &DocManifest) -> bool;
    fn create(
        &self,
        page_path: &PagePath,
        doc_path: &DocPath,
        manifest: &DocManifest,
    ) -> Result<Box<dyn Widget>, SemkaError>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocManifest {
    pub document_type: String,
}
