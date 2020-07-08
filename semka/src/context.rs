use crate::path;

pub struct Context {
    page_path_: path::PagePath,
    base_path_: path::AbsPath,
}

impl Context {
    pub fn new(page_path: path::PagePath, base_path: path::AbsPath) -> Self {
        Context {
            page_path_: page_path,
            base_path_: base_path,
        }
    }

    pub fn page_path(&self) -> &path::PagePath {
        &self.page_path_
    }
    pub fn base_path(&self) -> &path::AbsPath {
        &self.base_path_
    }
}
