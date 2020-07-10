use itertools::{EitherOrBoth, Itertools};
use std::ffi::OsStr;
use std::ops::Deref;
use std::path::{Path, PathBuf};

pub fn releative_path<P: Deref<Target = Path>>(base: &P, path: &P) -> PathBuf {
    let dir_up: &OsStr = "..".as_ref();
    let (up, down): (Vec<&OsStr>, Vec<&OsStr>) =
        base.iter()
            .zip_longest(path.iter())
            .fold((vec![], vec![]), |mut parts, pp| {
                match pp {
                    EitherOrBoth::Both(left, right) => {
                        if left != right {
                            parts.0.push(dir_up);
                            parts.1.push(right)
                        }
                    }
                    EitherOrBoth::Left(_) => parts.0.push(dir_up),
                    EitherOrBoth::Right(right) => parts.1.push(right),
                };
                parts
            });
    let rpath = PathBuf::new();
    let rpath = up.iter().fold(rpath, |mut path, p| {
        path.push(p);
        path
    });
    let rpath = down.iter().fold(rpath, |mut path, p| {
        path.push(p);
        path
    });
    rpath
}
