use crate::components::footer::Footer;
use crate::components::nav::Nav;
use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn PageTemplate(cx: Scope) -> Element {
  render! {
    div {
      class: "frame",
    div {
      class: "col-1",
    Nav { }
    }
    div {
      class: "col-2",
      Outlet::<Route> {}
    }
    div {
      class: "col-3",
    Footer { }
    }
    }
  }
}
