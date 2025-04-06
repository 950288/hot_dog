use crate::Route;
use dioxus::prelude::*;

const BLOG_CSS: Asset = asset!("/assets/styling/prot.scss");

#[component]
pub fn Prot() -> Element {
    let id = 1;
    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS}

        div {
            class: "prot container",
            h1 {
                class: "header",
                "Protein"
            }
            h2 {
                "Enzyme Commission Number Prediction"
            }

        }
    }
}
