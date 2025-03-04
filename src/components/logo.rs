use dioxus::prelude::*;
use crate::{Route};

const LOGO_SVG: &str = "https://obj.95028.top/website/95028-128px.png";
const LOGO_CSS: Asset = asset!("/assets/styling/logo.scss");

#[component]
pub fn Logo() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: LOGO_CSS }
        Link{
            id: "Logo",
            to: Route::Home,
            div {
                img { src: LOGO_SVG, id: "header" }
            }
        }
    }
}
