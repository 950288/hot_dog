use dioxus::prelude::*;
// use dioxus::web::get_root_element;


enum ColorModeType {
    light,
    dark,
}

pub static ColorMode: GlobalSignal<ColorModeType> = Global::new(|| ColorModeType::light);