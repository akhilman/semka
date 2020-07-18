use crate::builtin_widgets;
use crate::constants::DOC_DIR;
use crate::context::Context;
use crate::error::FetchError;
use crate::manifests::{DocManifest, SiteManifest};
use crate::path::Path;
use crate::utils;
use crate::widget::{Dependencies, Widget, WidgetCmd, WidgetMsg, WidgetOrders};
use enclose::enc;
use failure::{format_err, Error};
use futures::FutureExt;
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
        widgets: BTreeMap::new(),
        failed: BTreeSet::new(),
        dependencies: BTreeMap::new(),
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
    widgets: BTreeMap<Path, Box<dyn Widget>>,
    failed: BTreeSet<Path>,
    dependencies: BTreeMap<Path, BTreeSet<Path>>,
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
pub enum Msg {
    PageChanged(Path),
    SiteManifestChanged(SiteManifest),
    DocManifestFetched(Path, Result<DocManifest, FetchError>),
    WidgetReady(Path),
    WidgetFailed(Path, Error),
    WidgetMsg(Path, WidgetMsg),
    UpdateDependencies(Path, BTreeSet<Path>),
    DependenciesChanged(Path),
    Error(Error),
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>, ctx: &Context) {
    match msg {
        Msg::PageChanged(page) => {
            log!("PageChanged", page);
            update_current_page(model, orders, ctx);
        }
        Msg::Error(err) => error!(err),
        Msg::SiteManifestChanged(manifest) => {
            log!("SiteManifestChanged", manifest);
            update_current_page(model, orders, ctx);
        }
        Msg::DocManifestFetched(path, result) => {
            log!("DocManifestFetched", path, result);
            let result = result.map_err(Error::from).and_then(enc!(
                (path) | manifest | resolve_widget(path, manifest, ctx)
            ));
            match result {
                Ok(widget) => {
                    model.widgets.insert(path.clone(), widget);
                    orders.send_msg(Msg::WidgetReady(path));
                }
                Err(err) => {
                    orders.send_msg(Msg::WidgetFailed(path, err.into()));
                }
            }
        }
        Msg::WidgetReady(path) => {
            log!("WidgetReady", path);
            let result = model
                .widgets
                .get_mut(&path)
                .ok_or(format_err!("Widget for \"{}\" not found", &path))
                .and_then(|widget| widget.init(&path, ctx));
            handle_widget_result(result, path, orders);
        }
        Msg::WidgetFailed(path, error) => {
            log!("WidgetFailed", path, error);
            /*
            if let Some(fetch_err) = error.as_fail().downcast_ref::<FetchError>() {
                if fetch_err.is_not_found && path != NOT_FOUND_PATH {
                    TODO load 404 page
                }
            }
            */
            model.dependencies.remove(&path);
            model.widgets.insert(
                path.clone(),
                builtin_widgets::Failed::new(path.clone(), &error),
            );
            orders
                .send_msg(Msg::WidgetReady(path))
                .send_msg(Msg::Error(error));
        }
        Msg::WidgetMsg(path, msg) => {
            log!("WidgetMsg", path);
            if let Some(widget) = model.widgets.get_mut(&path) {
                let result = widget.update(msg, ctx);
                handle_widget_result(result, path, orders);
            } else {
                error!("Got message for unknown widget", path);
            }
        }
        Msg::UpdateDependencies(path, dependencies) => {
            log!("UpdateDependencies", path, dependencies);
            use std::collections::btree_map::Entry;
            match model.dependencies.entry(path.clone()) {
                Entry::Vacant(entry) => {
                    entry.insert(dependencies);
                    load_dependencies(&path, model, orders, ctx);
                    orders.send_msg(Msg::DependenciesChanged(path));
                }
                Entry::Occupied(mut entry) => {
                    if *entry.get() != dependencies {
                        entry.insert(dependencies);
                        load_dependencies(&path, model, orders, ctx);
                        orders.send_msg(Msg::DependenciesChanged(path));
                    } else {
                        orders.skip();
                    }
                }
            }
        }
        Msg::DependenciesChanged(path) => {
            log!("DependenciesChanged", path);
        }
    }
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
pub fn view(model: &Model, ctx: &Context) -> Node<Msg> {
    let doc_path = model.full_path.clone();
    Dependencies::new(&model.widgets, &model.dependencies, ctx)
        .view(&doc_path)
        .map_msg(move |msg| Msg::WidgetMsg(doc_path, msg))
}

// ------ ------
//     Misc
// ------ ------

fn update_current_page(model: &mut Model, orders: &mut impl Orders<Msg>, ctx: &Context) {
    model.page_path = current_page_or_index(ctx);
    model.full_path = ctx
        .site_manifest
        .master_page
        .clone()
        .join(model.page_path.clone());

    if !model.widgets.contains_key(&model.full_path) {
        load_document(model.full_path.clone(), model, orders, ctx);
    }
}

fn load_document(doc_path: Path, model: &mut Model, orders: &mut impl Orders<Msg>, ctx: &Context) {
    model
        .widgets
        .insert(doc_path.clone(), loading_widget(&doc_path, ctx));
    let fut = utils::fetch_doc_manifest(doc_path.head().to_string())
        .map(enc!((doc_path) move |result| {Msg::DocManifestFetched(doc_path, result)}));
    orders.perform_cmd(fut);
}

fn load_dependencies(
    doc_path: &Path,
    model: &mut Model,
    orders: &mut impl Orders<Msg>,
    ctx: &Context,
) {
    let to_load: Vec<Path> = model
        .dependencies
        .get(doc_path)
        .into_iter()
        .map(BTreeSet::iter)
        .flatten()
        .filter(|dep| !model.widgets.contains_key(dep))
        .cloned()
        .collect();
    for dep in to_load {
        load_document(dep, model, orders, ctx)
    }
}

fn resolve_widget(
    doc_path: Path,
    manifest: DocManifest,
    ctx: &Context,
) -> Result<Box<dyn Widget>, Error> {
    Ok(ctx
        .registry
        .get_widget(&manifest.widget)?
        .create(doc_path, manifest)?)
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

fn handle_widget_result(
    result: Result<Option<WidgetOrders>, Error>,
    path: Path,
    orders: &mut impl Orders<Msg>,
) {
    match result {
        Ok(Some(w_orders)) => {
            perform_widget_orders(w_orders, path, orders);
        }
        Ok(None) => (),
        Err(err) => {
            orders.send_msg(Msg::WidgetFailed(path, err));
        }
    }
}

fn perform_widget_orders(w_orders: WidgetOrders, doc_path: Path, orders: &mut impl Orders<Msg>) {
    w_orders.orders.into_iter().for_each(|cmd| {
        let doc_path = doc_path.clone();
        match cmd {
            WidgetCmd::FetchBytes(path) => {
                let full_file_path = Path::new().add(DOC_DIR).join(doc_path.head()).join(&path);
                let fut = utils::fetch_bytes(full_file_path).map(enc!((doc_path) move |result| {
                    Msg::WidgetMsg(doc_path.clone(), WidgetMsg::FetchBytesResult(path, result))
                }));
                orders.perform_cmd(fut);
            }
            WidgetCmd::FetchJson(path) => {
                let full_file_path = Path::new().add(DOC_DIR).join(doc_path.head()).join(&path);
                let fut = utils::fetch_json::<_, serde_json::Value>(full_file_path).map(
                    enc!((doc_path) move |result| {
                        Msg::WidgetMsg(doc_path.clone(), WidgetMsg::FetchJsonResult(path, result))
                    }),
                );
                orders.perform_cmd(fut);
            }
            WidgetCmd::FetchText(path) => {
                let full_file_path = Path::new().add(DOC_DIR).join(doc_path.head()).join(&path);
                let fut = utils::fetch_text(full_file_path).map(enc!((doc_path) move |result| {
                    Msg::WidgetMsg(doc_path.clone(), WidgetMsg::FetchTextResult(path, result))
                }));
                orders.perform_cmd(fut);
            }
            WidgetCmd::PerformCmd(fut) => {
                orders.perform_cmd(
                    fut.map(|result| Msg::WidgetMsg(doc_path, WidgetMsg::CmdResult(result))),
                );
            }
            WidgetCmd::UpdateDependencies(dependencies) => {
                orders.send_msg(Msg::UpdateDependencies(doc_path, dependencies));
            }
            WidgetCmd::Skip => {
                orders.skip();
            }
        }
    })
}

fn loading_widget(doc_path: &Path, ctx: &Context) -> Box<dyn Widget> {
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
