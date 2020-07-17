use crate::path::Path;
use crate::utils::is_url_absolute;
use seed::prelude::*;

pub trait NodeExt<T> {
    fn deep_map<F>(self, func: F) -> T
    where
        F: Fn(T) -> T + Copy;
    fn fold<F, O>(self, func: F) -> O
    where
        F: Fn(T, Vec<O>) -> O + Copy;
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

    fn fold<F, O>(self, func: F) -> O
    where
        F: Fn(Node<Ms>, Vec<O>) -> O + Copy,
    {
        match self {
            Node::Element(mut el) => {
                let mut children = vec![];
                std::mem::swap(&mut children, &mut el.children);
                let children: Vec<O> = children.into_iter().map(|child| child.fold(func)).collect();
                func(Node::Element(el), children)
            }
            node => func(node, vec![]),
        }
    }
}

pub(crate) fn to_absolute_href<Ms>(node: Node<Ms>, base_path: &Path) -> Node<Ms> {
    match node {
        Node::Element(mut el) => {
            if let Some(AtValue::Some(href)) = el.attrs.vals.get_mut(&At::Href) {
                if !is_url_absolute(&href) {
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
