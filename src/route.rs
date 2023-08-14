use crate::components::about::About;
use crate::components::colophon::Colophon;
use crate::components::home::Home;
use crate::components::page_not_found::PageNotFound;
use crate::components::page_template::PageTemplate;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Clone, Routable)]
pub enum Route {
  #[layout(PageTemplate)]
  #[route("/")]
  Home {},
  #[route("/about")]
  About {},
  #[route("/colophon")]
  Colophon {},
  #[end_layout]
  #[route("/:..route")]
  PageNotFound {
    route: Vec<String>,
  },
}
