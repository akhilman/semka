// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

// use crate::{path, widget};
use crate::error;
use crate::path;
// use crate::widget;
use seed::{prelude::*, *};
// use std::collections::BTreeMap;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
pub fn init(url: Url, orders: &mut impl Orders<AppMsg>) -> AppModel {
    let base_path: path::Path = orders.clone_base_path().iter().collect();
    let page_path: path::Path = url.path().iter().collect();
    let page_path = page_path.releative_to(&base_path);
    orders.perform_cmd(fetch_site_manifest());
    AppModel::new(page_path.into(), base_path.into())
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub struct AppModel {
    base_path: path::AbsPath,
    page_path: path::PagePath,
    site_manifest: Option<RootManifest>,
    // widgets: BTreeMap<path::DocPath, Box<dyn widget::Widget>>,
}

impl AppModel {
    pub fn new(page_path: path::PagePath, base_path: path::AbsPath) -> Self {
        // let widgets = BTreeMap::new();
        // AppModel { context, widgets }
        AppModel {
            page_path,
            base_path,
            site_manifest: None,
        }
    }
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone)]
// `Msg` describes the different events you can modify state with.
pub enum AppMsg {
    UrlChanged(Url),
    ReceivedRootManifest(RootManifest),
    ShowError(error::SemkaError),
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: AppMsg, model: &mut AppModel, orders: &mut impl Orders<AppMsg>) {
    match msg {
        AppMsg::UrlChanged(url) => log!(format!("UrlChanged({})", url)),
        AppMsg::ShowError(err) => log!(format!("Error {}", err)),
        AppMsg::ReceivedRootManifest(manifest) => {
            log!("ReceivedRootManifest", manifest);
            model.site_manifest = Some(manifest.clone());
        }
    }
}

fn fetch_site_manifest() -> impl futures::future::Future<Output = AppMsg> {
    static URL: &str = "site_manifest.json";
    async {
        let manifest = fetch(URL)
            .await?
            .check_status()?
            .json::<RootManifest>()
            .await?;
        Ok(manifest)
    }
    .map_ok(AppMsg::ReceivedRootManifest)
    .unwrap_or_else(|err: seed::browser::fetch::FetchError| {
        AppMsg::ShowError(error::FetchError::new(URL, err).into())
    })
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
pub fn view(model: &AppModel) -> Node<AppMsg> {
    div![
        C!["counter"],
        div![span!["Current path: "], span![model.page_path.to_string()],],
        div![span!["Base path: "], span![model.base_path.to_string()],],
        div![pre![
            serde_json::to_string_pretty(&model.site_manifest).unwrap()
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
