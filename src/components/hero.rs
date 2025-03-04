use dioxus::prelude::*;

const HEADER_SVG: &str = "https://obj.95028.top/doc_images/header/Starbase_Sunrise_Updade_Site_Photos_20240814_006731_1_c7a5980cec.jpg";

#[component]
pub fn Hero() -> Element {
    rsx! {
        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            // div { id: "links",
            //     a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
            //     a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
            //     a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
            //     a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
            //     a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
            //     a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            // }
        }
    }
}
