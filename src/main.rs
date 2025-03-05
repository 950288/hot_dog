use dioxus::prelude::*;
use routes::{Route};

mod components;
mod views;
mod routes;

const FAVICON: Asset = asset!("/assets/favicon.ico");
// const MAIN_CSS: Asset = ;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use document::Stylesheet;
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        Stylesheet { href: asset!("/assets/styling/main.scss")}
        Router::<Route> {}
    }
}
