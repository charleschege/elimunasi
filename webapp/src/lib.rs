#![allow(unused_braces)]
use log::{trace, Level};
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::HashChangeEvent;


#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod pages;
mod routes;

use routes::Credentials;

struct App {
    route: Credentials,
}

#[derive(Clone)]
enum AppModel {
    HashChange(String),
}

#[derive(Clone)]
enum AppView {
    PatchPage(Patch<View<HtmlElement>>),
    Error(String),
}

impl AppView {
    fn error(&self) -> Option<String> {
        match self {
            AppView::Error(msg) => Some(msg.clone()),
            _ => None,
        }
    }
    /// If the message is a new route, convert it into a patch to replace the current main page.
    fn patch_page(&self) -> Option<Patch<View<HtmlElement>>> {
        match self {
            AppView::PatchPage(patch) => Some(patch.clone()),
            _ => None,
        }
    }
}

impl Component for App {
    type DomNode = HtmlElement;
    type ModelMsg = AppModel;
    type ViewMsg = AppView;

    fn update(&mut self, msg: &AppModel, tx_view: &Transmitter<AppView>, _sub: &Subscriber<AppModel>) {
        match msg {
            AppModel::HashChange(hash) => {
                // When we get a hash change, attempt to convert it into one of our routes
                match Credentials::try_from(hash.as_str()) {
                    // Send error message to the view
                    Err(msg) => tx_view.send(&AppView::Error(msg)),
                    // Send patch message to the view
                    Ok(route) => {
                        if route != self.route {
                            let view = View::from(ViewBuilder::from(&route));
                            self.route = route;
                            tx_view.send(&AppView::PatchPage(Patch::Replace {
                                index: 0, //Index of html node
                                value: view,
                            }));
                        }
                    }
                }
            }
        }
    }

    fn view(&self, tx: &Transmitter<AppModel>, rx: &Receiver<AppView>) -> ViewBuilder<HtmlElement> {
        builder! {
            <slot
                window:hashchange=tx.contra_filter_map(|ev: &Event| {
                    let hev = ev.dyn_ref::<HashChangeEvent>().unwrap().clone();
                    let hash = hev.new_url();
                    Some(AppModel::HashChange(hash))
                })
                patch:children=rx.branch_filter_map(AppView::patch_page)>

                //<pre>{rx.branch_filter_map(AppView::error)}</pre>
                {ViewBuilder::from(&self.route)}
            </slot>
        }
    }
}

#[wasm_bindgen]
pub fn main(parent_id: Option<String>) -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();

    let gizmo = Gizmo::from(App { route: Credentials::RootPage });
    let view = View::from(gizmo.view_builder());

    if let Some(id) = parent_id {
        let parent = utils::document()
            .get_element_by_id(&id)
            .unwrap();

        view.run_in_container(&parent)
    } else {
        view.run()
    }
}