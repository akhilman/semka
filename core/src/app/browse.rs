use crate::builtin_widgets;
use crate::context::Context;
use crate::error::FetchError;
use crate::manifests::{DocManifest, SiteManifest};
use crate::path::Path;
use crate::utils;
use crate::widget;
use crate::widget::{Widget, WidgetMsg};
use enclose::enc;
use failure::{format_err, AsFail, Error};
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
            failed: BTreeSet::new(),
            loading: BTreeMap::new(),
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
    failed: BTreeSet<Path>,
    loading: BTreeMap<Path, CmdHandle>,
    deps: BTreeMap<Path, BTreeSet<Path>>,
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
pub enum Msg {
    PageChanged(Path),
    SiteManifestChanged(SiteManifest),
    DocManifestFetched(Path, Result<DocManifest, FetchError>),
    WidgetLoaded(Path),
    WidgetMsg(Path, WidgetMsg),
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
        Msg::DocManifestFetched(path, result) => {
            log!("DocManifestFetched", path, result);
            match result {
                Ok(manifest) => match load_widget(path.clone(), manifest, ctx) {
                    Ok(widget) => {
                        model
                            .widget_tree
                            .deps
                            .insert(path.clone(), widget.dependencies());
                        model.widget_tree.ready.insert(path.clone(), widget);
                        orders.send_msg(Msg::WidgetLoaded(path));
                    }
                    Err(err) => {
                        widget_failed(path, &err, model);
                        orders.send_msg(Msg::Error(err.into()));
                    }
                },
                Err(err) => {
                    widget_failed(path, &err, model);
                    orders.send_msg(Msg::Error(err.into()));
                }
            }
        }
        Msg::WidgetLoaded(path) => {
            log!("WidgetLoaded", path);
        }
        Msg::WidgetMsg(path, msg) => {
            log!("WidgetMsg", path);
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

    let full_path = &model.full_path;
    let widget_tree = &mut model.widget_tree;
    if !widget_tree.contains(full_path) {
        widget_tree
            .ready
            .insert(full_path.clone(), create_loading_widget(full_path, ctx));
        let fut = utils::fetch_doc_manifest(full_path.clone())
            .map(enc!((full_path) move |result| {Msg::DocManifestFetched(full_path, result)}));
        let handle = orders.perform_cmd_with_handle(fut);
        widget_tree.loading.insert(full_path.clone(), handle);
    }
}

fn load_widget(
    doc_path: Path,
    manifest: DocManifest,
    ctx: &Context,
) -> Result<Box<dyn Widget>, Error> {
    let factory = ctx.registry.get_widget(&manifest.widget)?;
    let widget = factory.create(doc_path, manifest)?;
    Ok(widget)
}

fn widget_failed(doc_path: Path, error: &impl AsFail, model: &mut Model) {
    let widget_tree = &mut model.widget_tree;
    widget_tree.failed.insert(doc_path.clone());
    widget_tree.loading.remove(&doc_path);
    widget_tree.deps.remove(&doc_path);
    widget_tree.ready.insert(
        doc_path.clone(),
        builtin_widgets::Failed::new(doc_path, error),
    );
}

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
        self.ready.contains_key(doc_path) || self.loading.contains_key(doc_path)
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
    let doc_path = model.full_path.clone();
    let maybe_root_widget = model.widget_tree.ready.get(&doc_path);
    div![
        C!["counter"],
        div![span!["Current path: "], span![model.page_path.to_string()],],
        div![span!["Full path: "], span![model.full_path.to_string()],],
        div![span!["Base path: "], span![ctx.base_path.to_string()],],
        div![pre![
            serde_json::to_string_pretty(&ctx.site_manifest).unwrap()
        ]],
        // button![model, ev(Ev::Click, |_| Msg::Increment),],
        match maybe_root_widget {
            Some(widget) => widget
                .view(ctx)
                .map_msg(|msg| Msg::WidgetMsg(doc_path, msg)),
            None => div![format!("No document with path \"{}\"", doc_path)],
        }
    ]
}

fn create_loading_widget(doc_path: &Path, ctx: &Context) -> Box<dyn Widget> {
    let manifest = DocManifest {
        widget: "loading".to_string(),
        ..DocManifest::default()
    };
    ctx.registry
        .get_widget("loading")
        .ok()
        .map(|factory| factory.create(doc_path.clone(), manifest).ok())
        .flatten()
        .unwrap_or_else(builtin_widgets::Loading::new)
}

// ------ ------
//     Misc
// ------ ------
