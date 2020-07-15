use crate::context::Context;
use crate::manifests::{DocManifest, SiteManifest};
use crate::path::Path;
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
pub fn init(_url: Url, _orders: &mut impl Orders<Msg>, ctx: &Context) -> Model {
    let page_path = current_page_or_index(ctx);
    let full_path = ctx
        .site_manifest
        .master_page
        .clone()
        .join(page_path.clone());
    Model {
        page_path,
        full_path,
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
    page_path: Path,
    full_path: Path,
    widget_tree: WidgetTree,
}

#[derive(Debug)]
struct WidgetTree {
    ready: BTreeMap<Path, Box<dyn Widget>>,
    failed: BTreeMap<Path, Error>,
    loading: BTreeMap<Path, CmdHandle>,
    pending: BTreeSet<Path>,
    deps: BTreeMap<Path, Vec<Path>>,
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
pub enum Msg {
    PageChanged(Path),
    SiteManifestChanged(SiteManifest),
    Error(Error),
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, ctx: &Context) {
    match msg {
        Msg::PageChanged(page) => {
            log!(format!("PageChanged({})", page));
            update_current_page(model, orders, ctx);
        }
        Msg::Error(err) => error!(err),
        Msg::SiteManifestChanged(manifest) => {
            log!("SiteManifestChanged", manifest);
            update_current_page(model, orders, ctx);
        }
    }
}

fn update_current_page(model: &mut Model, orders: &mut impl Orders<Msg>, ctx: &Context) {
    model.page_path = current_page_or_index(ctx);
    model.full_path = ctx
        .site_manifest
        .master_page
        .clone()
        .join(model.page_path.clone());

    /*
    let widget_tree = &mut model.widget_tree;
    if !widget_tree.contains(full_path) {
        utils::fetch_doc_manifest()
    }
    */
}

fn load_document(doc_path: Path, widget_tree: &mut WidgetTree) {}

fn current_page_or_index(ctx: &Context) -> Path {
    if !ctx.page_path.is_empty() {
        ctx.page_path.clone()
    } else if !ctx.site_manifest.index_page.is_empty() {
        ctx.site_manifest.index_page.clone()
    } else {
        "index".parse().unwrap()
    }
}

impl WidgetTree {
    fn contains(&self, doc_path: &Path) -> bool {
        self.ready.contains_key(doc_path)
            || self.failed.contains_key(doc_path)
            || self.loading.contains_key(doc_path)
    }
}

/*
async fn resolve_widget(
    doc_path: &path::Path,
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
        div![span!["Current path: "], span![model.page_path.to_string()],],
        div![span!["Full path: "], span![model.full_path.to_string()],],
        div![span!["Base path: "], span![ctx.base_path.to_string()],],
        div![pre![
            serde_json::to_string_pretty(&ctx.site_manifest).unwrap()
        ]],
        // button![model, ev(Ev::Click, |_| Msg::Increment),],
    ]
}

/*
fn loading() -> Node<Msg> {
    div!["Loading..."]
}

fn empty_site() -> Node<Msg> {
    div!["This site is empty"]
}
*/

// ------ ------
//     Misc
// ------ ------
