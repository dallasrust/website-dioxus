use super::components::about::About;
use super::components::benefits::Benefits;
use super::components::colophon::Colophon;
use super::components::home::Home;
use super::components::template::Template;
use ::dioxus::prelude::*;
use ::serde::{Deserialize, Serialize};

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Route {
  #[layout(Template)]
  #[route("/")]
  Home {},
  #[route("/about")]
  About {},
  #[route("/benefits")]
  Benefits {},
  #[route("/colophon")]
  Colophon {},
}
