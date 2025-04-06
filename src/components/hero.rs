use dioxus::prelude::*;

const HEADER_SVG: &str = "https://obj.95028.top/doc_images/header/Starbase_Sunrise_Updade_Site_Photos_20240814_006731_1_c7a5980cec.jpg";

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
        }
    }
}
