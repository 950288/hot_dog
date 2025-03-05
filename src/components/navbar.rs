use crate::components::Logo;
use std::collections::HashMap;
use std::rc::Rc;

use crate::routes::PAGE_DATA;
use crate::Route;
use dioxus::html::geometry::euclid::Size2D;
use dioxus::logger::tracing::info;
use dioxus::{html::geometry::euclid::Rect, prelude::*};

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.scss");

#[component]
pub fn Navbar() -> Element {
    let current_route = router().current::<Route>();
    let mut focused_route = use_signal(|| current_route.clone());

    let mut Link_elements: Signal<HashMap<Route, Rc<MountedData>>> = use_signal(|| HashMap::new());
    let mut onResize = use_signal(|| 0);

    let mut indicator_position: Signal<Rect<f64, dioxus_elements::geometry::Pixels>> =
        use_signal(|| Rect::zero());

    let indicator_position_res = use_resource(move || async move {
        onResize();
        if let Some(read) = Link_elements().get(&focused_route()) {
            if let Ok(rect) = (**read).get_client_rect().await {
                return rect;
            }
        }
        Rect::from_size(Size2D::new(0.0, 0.0))
    });

    use_effect(move || {
        if let Some(rect) = indicator_position_res() {
            indicator_position.set(rect);
        }
        info!("{:?}", indicator_position.read());
    });

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
                display: "flex",
                li {
                    class: "nav-logo",
                    Logo {}
                },
                li {
                    class: "indicator",
                    left: "{indicator_position().origin.x}px",
                    width: "{indicator_position().size.width}px"
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
    let current_route = Route::Calc;
    move || current_route.clone()
}
