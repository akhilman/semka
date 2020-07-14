use crate::context::{Context, Registry};
use crate::manifests::SiteManifest;
use crate::path::{AbsPath, PagePath};
use crate::utils;
use crate::widget::WidgetFactory;
use failure::{format_err, Error};
use seed::{prelude::*, *};

pub mod browse;
pub mod edit;

// ------ ------
//   Launcher
// ------ ------

pub struct Launcher {
    registry: Registry,
    root_element: Option<String>,
}

impl Launcher {
    pub fn new() -> Self {
        Self {
            registry: Registry::new(),
            root_element: None,
        }
    }

    pub fn add_widget<F>(self, factory: F) -> Self
    where
        F: WidgetFactory + 'static,
    {
        Self {
            registry: self.registry.add_widget(factory),
            ..self
        }
    }

    pub fn root_element(self, root_element: impl AsRef<str>) -> Self {
        Self {
            root_element: Some(root_element.as_ref().to_string()),
            ..self
        }
    }

    pub fn start(mut self) {
        let root_element = self.root_element.take().unwrap_or("app".to_string());
        let registry = self.registry;
        seed::App::start(
            root_element.as_str(),
            |url, orders| init(registry, url, orders),
            update,
            view,
        );
    }
}

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(registry: Registry, url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders
        .perform_cmd(
            utils::fetch_site_manifest()
                .map_ok_or_else(|err| Msg::ShowError(err.into()), Msg::SiteManifestChanged),
        )
        .subscribe(|url_changed: subs::UrlChanged| Msg::UrlChanged(url_changed.0));

    let base_path: AbsPath = orders
        .clone_base_path()
        .iter()
        .filter(|p| !p.is_empty())
        .collect();
    let page_path = url_to_page_path(&url, &base_path);
    let ctx = Context {
        url,
        page_path,
        base_path,
        site_manifest: SiteManifest::default(),
        registry,
    };

    Model {
        ctx,
        mode: Mode::Loading,
        browse: None,
        edit: None,
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.

#[derive(Debug)]
struct Model {
    ctx: Context,
    mode: Mode,
    browse: Option<browse::Model>,
    edit: Option<edit::Model>,
}

#[derive(Debug)]
enum Mode {
    Browse,
    Edit,
    Loading,
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
enum Msg {
    Browse(browse::Msg),
    Edit(edit::Msg),
    SiteManifestChanged(SiteManifest),
    ShowError(Error),
    UrlChanged(Url),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SiteManifestChanged(site_manifest) => {
            model.ctx.site_manifest = site_manifest;
            let site_manifest = &model.ctx.site_manifest;
            if let Some(browse_model) = &mut model.browse {
                browse::update(
                    browse::Msg::SiteManifestChanged(site_manifest.clone()),
                    browse_model,
                    &mut orders.proxy(Msg::Browse),
                    &model.ctx,
                )
            }
            if let Some(edit_model) = &mut model.edit {
                edit::update(
                    edit::Msg::SiteManifestChanged(site_manifest.clone()),
                    edit_model,
                    &mut orders.proxy(Msg::Edit),
                    &model.ctx,
                )
            }
            orders.notify(subs::UrlChanged(model.ctx.url.clone()));
        }
        Msg::UrlChanged(url) => {
            let page_path = url_to_page_path(&url, &model.ctx.base_path);
            let mode = path_to_mode(&page_path);

            model.ctx.page_path = page_path;
            model.ctx.url = url.clone();

            match mode {
                Mode::Edit => {
                    let mut edit_orders = orders.proxy(Msg::Edit);
                    let mut edit_model = model
                        .edit
                        .take()
                        .unwrap_or_else(|| edit::init(url.clone(), &mut edit_orders, &model.ctx));
                    edit::update(
                        edit::Msg::UrlChanged(url),
                        &mut edit_model,
                        &mut edit_orders,
                        &model.ctx,
                    );
                    model.edit.replace(edit_model);
                }
                Mode::Browse => {
                    let mut browse_orders = orders.proxy(Msg::Browse);
                    let mut browse_model = model.browse.take().unwrap_or_else(|| {
                        browse::init(url.clone(), &mut browse_orders, &model.ctx)
                    });
                    browse::update(
                        browse::Msg::UrlChanged(url),
                        &mut browse_model,
                        &mut browse_orders,
                        &model.ctx,
                    );
                    model.browse.replace(browse_model);
                }
                Mode::Loading => (),
            };
            model.mode = mode;
        }
        Msg::Browse(browse_msg) => {
            if let Some(browse_model) = &mut model.browse {
                browse::update(
                    browse_msg,
                    browse_model,
                    &mut orders.proxy(Msg::Browse),
                    &model.ctx,
                );
            } else {
                orders.send_msg(Msg::ShowError(format_err!("Browse mode not initialized")));
            }
        }
        Msg::Edit(edit_msg) => {
            if let Some(edit_model) = &mut model.edit {
                edit::update(
                    edit_msg,
                    edit_model,
                    &mut orders.proxy(Msg::Edit),
                    &model.ctx,
                );
            } else {
                orders.send_msg(Msg::ShowError(format_err!("Edit mode not initialized")));
            }
        }
        Msg::ShowError(err) => error!(err),
    }
}

fn path_to_mode(page_path: &PagePath) -> Mode {
    let first_part = page_path.iter().nth(0).unwrap_or("");
    match first_part {
        "_edit" => Mode::Edit,
        "_app" => Mode::About,
        _ => Mode::Browse,
    }
}

fn url_to_page_path(url: &Url, base_path: &AbsPath) -> PagePath {
    let abs_path: AbsPath = url.path().iter().collect();
    abs_path.releative_to(base_path).into()
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    match model.mode {
        Mode::Browse => {
            if let Some(browse_model) = &model.browse {
                browse::view(browse_model, &model.ctx).map_msg(Msg::Browse)
            } else {
                div!["View mode not initialized"]
            }
        }
        Mode::Edit => {
            if let Some(edit_model) = &model.edit {
                edit::view(edit_model, &model.ctx).map_msg(Msg::Edit)
            } else {
                div!["Edit mode not initialized"]
            }
        }
        Mode::Loading => div!["Loading..."],
    }
}

// ------ ------
//     Misc
// ------ ------