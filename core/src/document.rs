use crate::page::Context;
use failure_derive::Fail;
use seed::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::any::Any;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub trait Document {
    fn doc_path(&self) -> &Path;
    fn page_path(&self) -> &Path;
    fn manifest(&self) -> &Manifest;
    fn update(&mut self, msg: &Msg, context: &Context);
    fn view(&self, context: &Context) -> Node<Msg>;
}

pub trait Factory {
    fn can_handle(&self, manifest: &Manifest) -> bool;
    fn create(
        &self,
        page_path: &Path,
        doc_path: &Path,
        manifest: &Manifest,
        context: &Context,
    ) -> Result<Box<dyn Document>, Error>;
}

pub enum Msg {
    UrlChanged(Url),
    DocumentReady(PathBuf),
    Custom(PathBuf, Box<dyn Any>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Manifest {
    pub document_type: String,
}

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "The document \"{:?}\" has a bad manifest.", doc_path)]
    BadManifest { doc_path: PathBuf },
}
