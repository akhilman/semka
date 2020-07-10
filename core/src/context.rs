use crate::path::{DocPath, FilePath, PagePath};

pub struct Context<'a> {
    page_path: &'a PagePath,
    doc_path: &'a DocPath,
}

impl<'a> Context<'a> {
    pub fn new(page_path: &'a PagePath, doc_path: &'a DocPath) -> Self {
        Context {
            page_path,
            doc_path,
        }
    }
    pub fn fetch_binary(&mut self, path: FilePath) -> &mut Self {
        unimplemented!()
    }
    pub fn fetch_text(&mut self, path: FilePath) -> &mut Self {
        unimplemented!()
    }
    pub fn fetch_json<T>(&mut self, path: FilePath) -> &mut Self {
        unimplemented!()
    }
    pub fn skip(&mut self) -> &mut Self {
        unimplemented!()
    }
}
