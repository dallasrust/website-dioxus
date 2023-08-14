use crate::components::footer::Footer;
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
    div {
      class: "box",
    span {
      class: "drum",
      "Dallas Rust Users Meetup",
    }
    nav {
    ul {
    li {
      Link { to: Route::Home {}, "Home" }
    }
    li {
      Link { to: Route::About {}, "About" }
    }
    li {
      Link { to: Route::Colophon {}, "Colophon" }
    }
    }
    }
    }
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
