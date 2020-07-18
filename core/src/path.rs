use crate::error::{ParsePathError, PathError};
use itertools::{EitherOrBoth, Itertools};
use std::fmt::Write;
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq)]
struct PathInner {
    all_parts: String,
    ends_of_parts: Vec<usize>,
    absoulute: bool,
}

impl PathInner {
    fn new() -> Self {
        Self {
            all_parts: "".to_string(),
            ends_of_parts: vec![],
            absoulute: false,
        }
    }
    fn new_absolute() -> Self {
        Self {
            absoulute: true,
            ..Self::new()
        }
    }
    fn get(&self, index: usize) -> Option<&str> {
        let start = if index == 0 {
            0
        } else {
            self.ends_of_parts.get(index - 1).cloned()?
        };
        let end = self.ends_of_parts.get(index).cloned()?;
        self.all_parts.get(start..end)
    }
    fn push(&mut self, part: &str) {
        self.all_parts.push_str(part);
        self.ends_of_parts.push(self.all_parts.len())
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Path(Rc<PathInner>);

impl Path {
    pub fn new() -> Self {
        Self(Rc::new(PathInner::new()))
    }
    pub fn new_absolute() -> Self {
        Self(Rc::new(PathInner::new_absolute()))
    }
    pub fn iter(&self) -> PathIterator {
        PathIterator {
            inner: &*self.0,
            forward_i: 0,
            back_i: self.len(),
        }
    }
    pub fn add(&self, part: impl AsRef<str>) -> Self {
        let mut new_inner = (*self.0).clone();
        new_inner.push(part.as_ref());
        Self(Rc::new(new_inner))
    }
    pub fn join(&self, other: impl AsRef<Self>) -> Self {
        if other.as_ref().is_absolute() {
            other.as_ref().clone()
        } else if self.is_absolute() && self.is_empty() {
            let new_inner = PathInner {
                absoulute: true,
                ..(*other.as_ref().0).clone()
            };
            Self(Rc::new(new_inner))
        } else if other.as_ref().is_empty() {
            self.clone()
        } else {
            let mut new_inner = (*self.0).clone();
            other.as_ref().iter().for_each(|part| new_inner.push(part));
            Self(Rc::new(new_inner))
        }
    }
    pub fn head(&self) -> Self {
        if self.len() <= 1 {
            self.clone()
        } else {
            self.iter().take(1).collect()
        }
    }
    pub fn tail(&self) -> Self {
        self.iter().skip(1).collect()
    }
    pub fn releative_to(self, base: impl AsRef<Self>) -> Result<Self, PathError> {
        match (base.as_ref().is_absolute(), self.is_absolute()) {
            (true, true) | (false, false) => {
                Ok(releative_path(base.as_ref().iter(), self.iter()).collect())
            }
            _ => Err(PathError("Can not find releative path")),
        }
    }
    pub fn is_subpath(&self, rhs: impl AsRef<Self>) -> bool {
        match (self.is_absolute(), rhs.as_ref().is_absolute()) {
            (true, true) | (false, false) => is_subpath(self.iter(), rhs.as_ref().iter()),
            _ => false,
        }
    }
    pub fn is_superpath(&self, rhs: impl AsRef<Self>) -> bool {
        match (self.is_absolute(), rhs.as_ref().is_absolute()) {
            (true, true) | (false, false) => is_subpath(rhs.as_ref().iter(), self.iter()),
            _ => false,
        }
    }
    pub fn is_absolute(&self) -> bool {
        self.0.absoulute
    }
    pub fn is_empty(&self) -> bool {
        self.0.ends_of_parts.is_empty()
    }
    pub fn len(&self) -> usize {
        self.0.ends_of_parts.len()
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<Path> for Path {
    fn as_ref(&self) -> &Path {
        self
    }
}

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path(\"")?;
        std::fmt::Display::fmt(self, f)?;
        write!(f, "\")")?;
        Ok(())
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_absolute() {
            f.write_char('/')?;
        }
        self.iter().format("/").fmt(f)
    }
}

impl std::str::FromStr for Path {
    type Err = ParsePathError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path: Path = s.split('/').filter(|s| !s.is_empty()).collect();
        Ok(if s.starts_with("/") {
            Path::new_absolute().join(path)
        } else {
            path
        })
    }
}

impl std::ops::Index<usize> for Path {
    type Output = str;
    fn index(&self, index: usize) -> &str {
        self.0.get(index).expect("Index in range")
    }
}

pub struct PathIterator<'a> {
    inner: &'a PathInner,
    forward_i: usize,
    back_i: usize,
}

impl<'a> Iterator for PathIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.forward_i < self.back_i {
            let i = self.forward_i;
            self.forward_i += 1;
            self.inner.get(i)
        } else {
            None
        }
    }
}
impl<'a> DoubleEndedIterator for PathIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.forward_i < self.back_i {
            self.back_i -= 1;
            self.inner.get(self.back_i)
        } else {
            None
        }
    }
}

impl<S> std::iter::FromIterator<S> for Path
where
    S: std::convert::AsRef<str>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = S>,
    {
        Self(Rc::new(iter.into_iter().fold(
            PathInner::new(),
            |mut path, part| {
                path.push(part.as_ref());
                path
            },
        )))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        for parts in self.iter().zip_longest(other.iter()) {
            match parts {
                EitherOrBoth::Both(left, right) => match left.cmp(right) {
                    Ordering::Equal => continue,
                    ord => return ord,
                },
                EitherOrBoth::Left(_) => return Ordering::Greater,
                EitherOrBoth::Right(_) => return Ordering::Less,
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl serde::ser::Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> serde::de::Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Path, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(PathVisitor)
    }
}

struct PathVisitor;
impl<'de> serde::de::Visitor<'de> for PathVisitor {
    type Value = Path;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("string path")
    }
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        value.parse().map_err(|e| serde::de::Error::custom(e))
    }
}

pub fn releative_path<'a, B, P>(base: B, path: P) -> impl Iterator<Item = &'a str>
where
    B: std::iter::Iterator<Item = &'a str>,
    P: std::iter::Iterator<Item = &'a str>,
{
    static DIR_UP: &str = "..";
    let (up, down): (Vec<&'a str>, Vec<&'a str>) =
        base.zip_longest(path)
            .fold((vec![], vec![]), |mut parts, pp| {
                match pp {
                    EitherOrBoth::Both(left, right) => {
                        if left != right {
                            parts.0.push(DIR_UP);
                            parts.1.push(right)
                        }
                    }
                    EitherOrBoth::Left(_) => parts.0.push(DIR_UP),
                    EitherOrBoth::Right(right) => parts.1.push(right),
                };
                parts
            });
    up.into_iter().chain(down.into_iter())
}

pub fn is_subpath<'a, I>(lhs: I, rhs: I) -> bool
where
    I: std::iter::Iterator<Item = &'a str>,
{
    for part in lhs.into_iter().zip_longest(rhs.into_iter()) {
        match part {
            EitherOrBoth::Both(left, right) => {
                if left != right {
                    return false;
                }
            }
            EitherOrBoth::Left(_) => return true,
            EitherOrBoth::Right(_) => return false,
        }
    }
    false // lhs == rhs
}
