use dioxus::prelude::*;
use super::components::Navbar;
use super::views::{Blog, Home, Calc};

#[derive(Debug, Clone, Routable, PartialEq, Eq, Hash)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home,
    #[route("/blog/")]
    Blog,
    #[route("/calc/")]
    Calc,
}

#[derive(Debug, Clone)]
pub struct PageContent {
    pub title: String
}


use indexmap::IndexMap;
use once_cell::sync::Lazy;
pub(crate) static PAGE_DATA: Lazy<IndexMap<Route, PageContent>> = Lazy::new(|| {
    let mut map = IndexMap::new();
    map.insert(Route::Home, PageContent { title: "Home".to_string() });
    map.insert(Route::Blog, PageContent { title: "Blog".to_string() });
    map.insert(Route::Calc, PageContent { title: "Calc".to_string() });
    map
});