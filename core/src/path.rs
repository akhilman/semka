use crate::error::{ParsePathError, PathError};
use itertools::{EitherOrBoth, Itertools};
use std::fmt::Write;

#[derive(Clone, Eq, PartialEq)]
pub struct Path {
    all_parts: String,
    ends_of_parts: Vec<usize>,
    absoulute: bool,
}

impl Path {
    pub fn new() -> Self {
        Self {
            all_parts: "".to_string(),
            ends_of_parts: vec![],
            absoulute: false,
        }
    }
    pub fn new_absolute() -> Self {
        Self {
            all_parts: "".to_string(),
            ends_of_parts: vec![],
            absoulute: true,
        }
    }
    pub fn iter(&self) -> PathIterator {
        PathIterator {
            path: self,
            forward_i: 0,
            back_i: self.len(),
        }
    }
    pub fn push(&mut self, part: &str) {
        self.all_parts.push_str(part);
        self.ends_of_parts.push(self.all_parts.len())
    }
    pub fn append(&mut self, other: &Self) {
        if other.is_absolute() {
            std::mem::swap(self, &mut other.clone());
        } else {
            other.iter().for_each(|part| self.push(part));
        }
    }
    pub fn add(mut self, part: &str) -> Self {
        self.push(part);
        self
    }
    pub fn join(mut self, other: Self) -> Self {
        if other.is_absolute() {
            other
        } else if self.is_absolute() && self.is_empty() {
            Self {
                absoulute: true,
                ..other
            }
        } else {
            self.append(&other);
            self
        }
    }
    pub fn releative_to(self, base: &Self) -> Result<Self, PathError> {
        match (base.is_absolute(), self.is_absolute()) {
            (true, true) | (false, false) => {
                let mut new = Self::new();
                releative_path(base.iter(), self.iter()).for_each(|part| new.push(part));
                Ok(new)
            }
            _ => Err(PathError("Can not find releative path")),
        }
    }
    pub fn is_subpath(&self, rhs: &Self) -> bool {
        match (self.is_absolute(), rhs.is_absolute()) {
            (true, true) | (false, false) => is_subpath(self.iter(), rhs.iter()),
            _ => false,
        }
    }
    pub fn is_superpath(&self, rhs: &Self) -> bool {
        rhs.is_subpath(self)
    }
    pub fn is_absolute(&self) -> bool {
        self.absoulute
    }
    pub fn is_empty(&self) -> bool {
        self.ends_of_parts.is_empty()
    }
    pub fn len(&self) -> usize {
        self.ends_of_parts.len()
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
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
        let path = s.split('/').filter(|s| !s.is_empty()).collect();
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
        let start = if index == 0 {
            0
        } else {
            self.ends_of_parts[index - 1]
        };
        let end = self.ends_of_parts[index];
        &self.all_parts[start..end]
    }
}

pub struct PathIterator<'a> {
    path: &'a Path,
    forward_i: usize,
    back_i: usize,
}

impl<'a> Iterator for PathIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.forward_i < self.back_i {
            let i = self.forward_i;
            self.forward_i += 1;
            Some(&self.path[i])
        } else {
            None
        }
    }
}
impl<'a> DoubleEndedIterator for PathIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.forward_i < self.back_i {
            self.back_i -= 1;
            Some(&self.path[self.back_i])
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
        iter.into_iter().fold(Path::new(), |mut path, part| {
            path.push(part.as_ref());
            path
        })
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
        unreachable!()
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
