use std::collections::HashMap;
use crate::components::{Logo};

use crate::{Route};
use dioxus::logger::tracing::info;
use dioxus::{html::geometry::euclid::Rect, prelude::*};
use crate::routes::PAGE_DATA;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.scss");

#[component]
pub fn Navbar() -> Element {
    let current_route =  router().current::<Route>();
    let mut focused_route =  use_signal(|| current_route.clone());

    let mut Link_elements: Signal<HashMap<Route, std::rc::Rc<MountedData>>> = use_signal(|| HashMap::new());
    let mut onResize = use_signal(|| 0);

    let indicator_position = use_resource(move || async move {
        onResize();
        let Link_elements = Link_elements.read();
        let read = Link_elements.get(&focused_route());
        let client_rect = read.as_ref().map(|el| el.get_client_rect());
        if let Some(client_rect) = client_rect {
            if let Ok(rect) = client_rect.await {
                return rect
            } else {
                Rect::zero()
            }
        } else {
            Rect::zero()
        }
    });

    // let car = format!("{:?}", indicator_position());

    // let value = &current_route;
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        nav {
            id: "navbar",
            onresize: move |_| {
                onResize.set(onResize() + 1);
            },
            ul {
                id: "navbar-list",
                class: "col-md-22 col-20",
                li {
                    class: "nav-logo",
                    Logo {}
                },
                li {
                    left: "{indicator_position().unwrap_or(Rect::zero()).origin.x}px",
                    width: "{indicator_position().unwrap_or(Rect::zero()).size.width}px",
                    class: "indicator",
                },
                for (route, content) in &*PAGE_DATA {
                    li { 
                        class: "navbar-item",
                        onmounted: move |element| {
                            let mut Link_elements_clone = Link_elements();
                            Link_elements_clone.insert(route.clone(), element.data());
                            Link_elements.set(Link_elements_clone);
                        },
                        onmouseover: move |_| {
                            focused_route.set(route.clone());
                        },
                        onmouseout: {
                            move |_| {
                            focused_route.set(router().current::<Route>())
                        }},
                        Link {
                            class: "navbar-link",
                            to: route.clone(),
                            "{content.title}"
                        }
                    }
                }
            }
        }
        Outlet::<Route> {}
    }
}

fn test() -> impl FnMut() -> Route {
    let current_route =  Route::Calc;
    move || current_route.clone()
}