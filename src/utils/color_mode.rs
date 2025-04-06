use dioxus::{logger::tracing, prelude::*};
use web_sys::window;
// use dioxus::web::get_root_element;

#[derive(PartialEq,Debug,Clone, Copy)]
pub enum ColorModeType {
    light,
    dark,
}

impl ColorModeType {
    pub fn as_str(&self) -> &str {
        match self {
            ColorModeType::light => "light",
            ColorModeType::dark => "dark",
        }
    }


}

pub static ColorMode: GlobalSignal<ColorModeType> = Global::new(|| ColorModeType::light);