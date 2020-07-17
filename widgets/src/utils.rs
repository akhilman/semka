use lazy_static::lazy_static;
use seed::{prelude::*, *};
use semka_core::prelude::*;
use std::collections::BTreeSet;

lazy_static! {
    static ref DOC_AT: At = At::Custom("doc".into());
    static ref INCLUDE_TAG: Tag = Tag::Custom("include".into());
}

pub(crate) fn find_include_deps(node: Node<WidgetMsg>) -> BTreeSet<Path> {
    node.flatten()
        .into_iter()
        .filter_map(|node| match node {
            Node::Element(el) => {
                if el.tag == *INCLUDE_TAG {
                    el.attrs
                        .vals
                        .get(&*DOC_AT)
                        .map(|at| {
                            if let AtValue::Some(doc) = at {
                                doc.parse().ok()
                            } else {
                                None
                            }
                        })
                        .flatten()
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect()
}

pub(crate) fn resolve_include(
    node: Node<WidgetMsg>,
    dependencies: Dependencies,
) -> Node<WidgetMsg> {
    match node {
        Node::Element(el) => {
            if el.tag == *INCLUDE_TAG {
                if let Some(AtValue::Some(doc)) = el.attrs.vals.get(&*DOC_AT) {
                    if let Ok(doc_path) = doc.parse::<Path>() {
                        dependencies.view(&doc_path)
                    } else {
                        error!("Can not parse \"doc\" attribute of include element");
                        div![doc]
                    }
                } else {
                    error!("Include element has no \"doc\" attribute");
                    empty!()
                }
            } else {
                Node::Element(el)
            }
        }
        node => node,
    }
}
