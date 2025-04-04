use dioxus::{document::{Document, Stylesheet}, prelude::*};
#[cfg(feature = "web")]
use routes::Route;
use utils::color_mode::ColorMode;

mod components;
mod utils;
#[cfg(feature = "web")]
mod routes;
#[cfg(feature = "web")]
mod views;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    #[cfg(feature = "web")]
    rsx! {
        document::Stylesheet { href: asset!("/assets/styling/main.scss")}
        document::Link { rel: "icon", href: "data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>🚀</text></svg>" }
        Router::<Route> {}
    }
    #[cfg(not(feature = "web"))]
    rsx!( div { "Hello, world" } )
}
