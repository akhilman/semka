use crate::path::FilePath;

pub struct Context {}

impl Context {
    pub fn new() -> Self {
        Context {}
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
