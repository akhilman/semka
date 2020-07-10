use itertools::{EitherOrBoth, Itertools};
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq)]
pub struct Path {
    all_parts: String,
    ends_of_parts: Vec<usize>,
}

impl Path {
    pub fn new() -> Self {
        Self {
            all_parts: "".to_string(),
            ends_of_parts: vec![],
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
        other.iter().for_each(|part| self.push(part))
    }

    pub fn releative_to(&self, base: &Self) -> Self {
        let mut new = Self::new();
        releative_path(base.iter(), self.iter())
            .into_iter()
            .for_each(|part| new.push(part));
        new
    }
    pub fn is_subpath(&self, rhs: &Self) -> bool {
        is_subpath(self.iter(), rhs.iter())
    }
    pub fn is_superpath(&self, rhs: &Self) -> bool {
        is_subpath(rhs.iter(), self.iter())
    }

    pub fn len(&self) -> usize {
        self.ends_of_parts.len()
    }
}

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"")?;
        std::fmt::Display::fmt(self, f)?;
        write!(f, "\"")?;
        Ok(())
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.iter().format("/").fmt(f)
    }
}

#[derive(failure::Fail, Debug)]
#[fail(display = "Can not parse path: {}", _0)]
pub struct ParsePathError(&'static str);

impl std::str::FromStr for Path {
    type Err = ParsePathError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.split('/').collect())
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

macro_rules! path_variant {
    ($t:ident) => {
        #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
        pub struct $t(Path);

        impl $t {
            pub fn new() -> Self {
                Self(Path::new())
            }
            pub fn iter(&self) -> PathIterator {
                self.0.iter()
            }

            pub fn push(&mut self, part: &str) {
                self.0.push(part)
            }

            pub fn append(&mut self, other: &mut Self) {
                self.0.append(&other.0)
            }

            pub fn releative_to(&self, base: &Self) -> Self {
                Self(self.0.releative_to(&base.0))
            }
            pub fn is_subpath(&self, rhs: &Self) -> bool {
                self.0.is_subpath(&rhs.0)
            }
            pub fn is_superpath(&self, rhs: &Self) -> bool {
                self.0.is_superpath(&rhs.0)
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }
        }

        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::str::FromStr for $t {
            type Err = ParsePathError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Path::from_str(s).map(Self)
            }
        }

        impl std::ops::Index<usize> for $t {
            type Output = str;
            fn index(&self, index: usize) -> &str {
                self.0.index(index)
            }
        }

        impl<S> std::iter::FromIterator<S> for $t
        where
            S: std::convert::AsRef<str>,
        {
            fn from_iter<T>(iter: T) -> Self
            where
                T: IntoIterator<Item = S>,
            {
                Self(Path::from_iter(iter))
            }
        }

        impl std::convert::From<Path> for $t {
            fn from(path: Path) -> Self {
                Self(path)
            }
        }

        impl std::convert::From<$t> for Path {
            fn from(path: $t) -> Self {
                path.0
            }
        }
    };
}

path_variant!(DocPath);
path_variant!(FilePath);
path_variant!(PagePath);

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct AbsPath(Path);

impl AbsPath {
    pub fn new() -> Self {
        Self(Path::new())
    }
    pub fn iter(&self) -> PathIterator {
        self.0.iter()
    }

    pub fn push(&mut self, part: &str) {
        self.0.push(part)
    }

    pub fn releative_to(&self, base: &Self) -> Path {
        self.0.releative_to(&base.0)
    }

    pub fn as_url(&self) -> seed::Url {
        self.iter()
            .fold(seed::Url::new(), |url, part| url.add_path_part(part))
    }
}

impl std::fmt::Display for AbsPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::iter::once("").chain(self.iter()).format("/").fmt(f)
    }
}

impl std::str::FromStr for AbsPath {
    type Err = ParsePathError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().nth(0) == Some('/') {
            Ok(s.split('/').collect())
        } else {
            Err(ParsePathError("Path is not absolute"))
        }
    }
}

impl std::ops::Index<usize> for AbsPath {
    type Output = str;
    fn index(&self, index: usize) -> &str {
        self.0.index(index)
    }
}

impl<S> std::iter::FromIterator<S> for AbsPath
where
    S: std::convert::AsRef<str>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = S>,
    {
        Self(Path::from_iter(iter))
    }
}

impl std::convert::From<Path> for AbsPath {
    fn from(path: Path) -> Self {
        Self(path)
    }
}

impl std::convert::From<AbsPath> for Path {
    fn from(path: AbsPath) -> Self {
        path.0
    }
}

impl serde::ser::Serialize for AbsPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> serde::de::Deserialize<'de> for AbsPath {
    fn deserialize<D>(deserializer: D) -> Result<AbsPath, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(AbsPathVisitor)
    }
}

struct AbsPathVisitor;
impl<'de> serde::de::Visitor<'de> for AbsPathVisitor {
    type Value = AbsPath;
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

fn releative_path<'a, I>(base: I, path: I) -> Vec<&'a str>
where
    I: std::iter::Iterator<Item = &'a str>,
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
    up.into_iter().chain(down.into_iter()).collect()
}

fn is_subpath<'a, I>(lhs: I, rhs: I) -> bool
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
