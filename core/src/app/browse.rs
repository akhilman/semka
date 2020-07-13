use crate::context::Context;
use crate::manifests::{DocManifest, SiteManifest};
use crate::path;
use crate::path::DocPath;
use crate::utils;
use crate::widget;
use crate::widget::Widget;
use failure::{format_err, Error};
use futures::TryFutureExt;
use seed::{prelude::*, *};
use std::collections::{BTreeMap, BTreeSet};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
pub fn init(url: Url, orders: &mut impl Orders<Msg>, ctx: &Context) -> Model {
    Model {
        current: None,
        master: None,
        widget_tree: WidgetTree {
            ready: BTreeMap::new(),
            failed: BTreeMap::new(),
            loading: BTreeMap::new(),
            pending: BTreeSet::new(),
            deps: BTreeMap::new(),
        },
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
#[derive(Debug)]
pub struct Model {
    current: Option<DocPath>,
    master: Option<DocPath>,
    widget_tree: WidgetTree,
}

#[derive(Debug)]
struct WidgetTree {
    ready: BTreeMap<DocPath, Box<dyn Widget>>,
    failed: BTreeMap<DocPath, Error>,
    loading: BTreeMap<DocPath, CmdHandle>,
    pending: BTreeSet<DocPath>,
    deps: BTreeMap<DocPath, Vec<DocPath>>,
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
pub enum Msg {
    UrlChanged(Url),
    SiteManifestChanged(SiteManifest),
    Error(Error),
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, ctx: &Context) {
    match msg {
        Msg::UrlChanged(url) => log!(format!("UrlChanged({})", url)),
        Msg::Error(err) => error!(err),
        Msg::SiteManifestChanged(manifest) => {
            log!("SiteManifestChanged", manifest);
        }
    }
}

/*
async fn resolve_widget(
    doc_path: &path::DocPath,
    reg: &register::Register,
) -> Result<Box<dyn widget::Widget>, Error> {
    let manifest = utils::fetch_doc_manifest(doc_path).await?;
    let factory = reg.resolve(&manifest)?;
    let widget = factory.create(manifest, doc_path.clone())?;
    Ok(widget)
}
*/

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
    div![
        C!["counter"],
        div![span!["Current path: "], span![ctx.page_path.to_string()],],
        div![span!["Base path: "], span![ctx.base_path.to_string()],],
        div![pre![
            serde_json::to_string_pretty(&ctx.site_manifest).unwrap()
        ]],
        // button![model, ev(Ev::Click, |_| Msg::Increment),],
    ]
}

fn loading() -> Node<Msg> {
    div!["Loading..."]
}

fn empty_site() -> Node<Msg> {
    div!["This site is empty"]
}

// ------ ------
//     Misc
// ------ ------
