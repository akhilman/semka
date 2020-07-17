use crate::path::Path;
use lazy_static::lazy_static;
use regex::Regex;
use seed::prelude::*;

pub trait NodeExt<T> {
    fn deep_map<F>(self, func: F) -> T
    where
        F: Fn(T) -> T + Copy;
    fn flatten(self) -> Vec<T>;
}

impl<Ms: 'static> NodeExt<Node<Ms>> for Node<Ms> {
    fn deep_map<F>(self, func: F) -> Node<Ms>
    where
        F: Fn(Node<Ms>) -> Node<Ms> + Copy,
    {
        match func(self) {
            Node::Element(mut el) => {
                let new_children = el
                    .children
                    .into_iter()
                    .map(move |child| child.deep_map(func))
                    .collect();
                el.children = new_children;
                Node::Element(el)
            }
            node => node,
        }
    }

    fn flatten(self) -> Vec<Node<Ms>> {
        match self {
            Node::Element(mut el) => {
                let mut children = vec![];
                std::mem::swap(&mut children, &mut el.children);
                let nodes: Vec<Node<Ms>> = children
                    .into_iter()
                    .map(|node| node.flatten().into_iter())
                    .flatten()
                    .chain(std::iter::once(Node::Element(el)))
                    .collect();
                nodes
            }
            node => vec![node],
        }
    }
}

pub(crate) fn to_absolute_href<Ms>(node: Node<Ms>, base_path: &Path) -> Node<Ms> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(/|[\w]+:|[\w]+//).*$").unwrap();
    }
    match node {
        Node::Element(mut el) => {
            if let Some(AtValue::Some(href)) = el.attrs.vals.get_mut(&At::Href) {
                if !RE.is_match(href) {
                    if let Ok(path) = href.parse::<Path>() {
                        *href = base_path.join(path).to_string();
                    }
                }
            }
            Node::Element(el)
        }
        node => node,
    }
}
