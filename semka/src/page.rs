// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use crate::context::Context;
// use crate::{path, widget};
use crate::error;
use crate::path;
use seed::{prelude::*, *};
// use std::collections::BTreeMap;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
pub(crate) fn init(url: Url, orders: &mut impl Orders<PageMsg>) -> PageModel {
    let base_path: path::Path = orders.clone_base_path().iter().collect();
    let page_path: path::Path = url.path().iter().collect();
    let page_path = page_path.releative_to(&base_path);
    orders.send_msg(PageMsg::FetchRootManifest);
    PageModel::new(Context::new(page_path.into(), base_path.into()))
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub(crate) struct PageModel {
    context: Context,
    root_manifest: Option<RootManifest>, // widgets: BTreeMap<path::DocPath, Box<dyn widget::Widget>>,
}

impl PageModel {
    fn new(context: Context) -> Self {
        // let widgets = BTreeMap::new();
        // PageModel { context, widgets }
        PageModel {
            context,
            root_manifest: None,
        }
    }
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
// `Msg` describes the different events you can modify state with.
pub enum PageMsg {
    UrlChanged(Url),
    FetchRootManifest,
    ReceivedRootManifest(RootManifest),
    ShowError(error::SemkaError),
}

// `update` describes how to handle each `Msg`.
pub(crate) fn update(msg: PageMsg, model: &mut PageModel, orders: &mut impl Orders<PageMsg>) {
    match msg {
        PageMsg::UrlChanged(url) => log!(format!("UrlChanged({})", url)),
        PageMsg::ShowError(err) => log!(format!("Error {}", err)),
        PageMsg::FetchRootManifest => {
            orders.perform_cmd(fetch_root_manifest(&model.context.base_path()));
        }
        PageMsg::ReceivedRootManifest(manifest) => {
            log!("ReceivedRootManifest", manifest);
            model.root_manifest = Some(manifest.clone());
        }
    }
}

fn fetch_root_manifest(
    base_path: &path::AbsPath,
) -> impl futures::future::Future<Output = PageMsg> {
    let mut manifest_path = base_path.clone();
    manifest_path.push("manifest.json");
    let url = manifest_path.as_url();
    async {
        let manifest = fetch(url)
            .await?
            .check_status()?
            .json::<RootManifest>()
            .await?;
        Ok(manifest)
    }
    .map_ok(PageMsg::ReceivedRootManifest)
    .unwrap_or_else(|err: seed::browser::fetch::FetchError| {
        PageMsg::ShowError(error::FetchError::new(manifest_path, format!("{:?}", err)).into())
    })
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
pub(crate) fn view(model: &PageModel) -> Node<PageMsg> {
    let context = &model.context;
    div![
        C!["counter"],
        div![
            span!["Current path: "],
            span![context.page_path().to_string()],
        ],
        div![span!["Base path: "], span![context.base_path().to_string()],],
        div![pre![
            serde_json::to_string_pretty(&model.root_manifest).unwrap()
        ]],
        // button![model, ev(Ev::Click, |_| Msg::Increment),],
    ]
}

// ------ ------
//     Misc
// ------ ------

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootManifest {
    root_document: path::DocPath,
}

impl Default for RootManifest {
    fn default() -> Self {
        Self {
            root_document: "emptySite".parse().unwrap(),
        }
    }
}

