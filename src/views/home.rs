use crate::components::{Echo, Hero};
use dioxus::prelude::*;

const HOME_CSS: Asset = asset!("/assets/styling/home.scss");

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS },
        div {
            class: "container",
            Hero {}
            Echo {}
        }
    }
}
