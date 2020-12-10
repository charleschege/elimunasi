use crate::{Route, route_dispatch};
use mogwai::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum Out {
    Render { route: Route },
}

impl Out {
    fn maybe_patch_route(&self) -> Option<Patch<View<HtmlElement>>> {
        if let Out::Render { route } = self {
            Some(Patch::Replace {
                index: 0,
                value: View::from(ViewBuilder::from(route)),
            })
        } else {
            None
        }
    }
}

pub struct App {
    current_route: Route,
}

impl App {
    pub fn gizmo(initial_route: Route) -> Gizmo<Self> {
        let app = App {
            current_route: initial_route
        };
        Gizmo::from(app)
    }
}

impl Component for App {
    type ModelMsg = Route;
    type ViewMsg = Out;
    type DomNode = HtmlElement;

    fn update(&mut self, msg: &Route, tx_view: &Transmitter<Out>, _sub: &Subscriber<Route>) {
        self.current_route = *msg;
        tx_view.send(&Out::Render {
            route: self.current_route,
        });
        route_dispatch::push_state(*msg);
    }

    fn view(&self, tx: &Transmitter<Route>, rx: &Receiver<Out>) -> ViewBuilder<HtmlElement> {
        builder! {
            <div id="root" class="root">
                <nav>
                <a
                        href="/"
                        style="margin-right: 15px;"
                        on:click=tx.contra_map(|e: &Event| {
                            e.prevent_default();
                            Route::Home
                        })
                    >
                        "Home"
                    </a>
                    <a
                        href="/login"
                        style="margin-right: 15px;"
                        on:click=tx.contra_map(|e: &Event| {
                            e.prevent_default();
                            Route::Home
                        })
                    >
                        "Login"
                    </a>
                    <a
                        href="/logout"
                        style="margin-right: 15px;"
                        on:click=tx.contra_map(|e: &Event| {
                            e.prevent_default();
                            Route::Home
                        })
                    >
                        "Logout"
                    </a>
                    <a
                        href="/404"
                        style="margin-right: 15px;"
                        on:click=tx.contra_map(|e: &Event| {
                            e.prevent_default();
                            Route::NotFound
                        })
                    >
                        "Not Found"
                    </a>
                </nav>
                <slot patch:children=rx.branch_filter_map(Out::maybe_patch_route)>
                    {ViewBuilder::from(&self.current_route)}
                </slot>
            </div>
        }
    }
}