use dioxus::prelude::*;
use routes::{Route, CURRENT_ROUTE};
use dioxus::router;

mod components;
mod views;
mod routes;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const COL_CSS: Asset = asset!("/assets/styling/col.scss");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: COL_CSS }

        Router::<Route> {}
    }
}
