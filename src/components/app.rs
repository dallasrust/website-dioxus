use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
  render! {
    Router::<Route> { }
  }
}
