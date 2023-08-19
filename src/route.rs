use crate::components::about::About;
use crate::components::colophon::Colophon;
use crate::components::home::Home;
use crate::components::page_not_found::PageNotFound;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Clone, Routable)]
pub enum Route {
  #[route("/")]
  Home {},
  #[route("/about")]
  About {},
  #[route("/colophon")]
  Colophon {},
  #[route("/:..route")]
  PageNotFound {
    route: Vec<String>,
  },
}
