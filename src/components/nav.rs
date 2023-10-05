use crate::components::animation::Animation;
use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn Nav(cx: Scope) -> Element {
  render! {
    nav {
      class: "app-nav box",
    Animation {}
    ul {
    li {
    Link {
      active_class: "app-nav-active",
      to: Route::Home {},
      "Home",
    }
    }
    li {
    Link {
      active_class: "app-nav-active",
      to: Route::About {},
      "About",
    }
    }
    li {
    Link {
      active_class: "app-nav-active",
      to: Route::Colophon {},
      "Colophon",
    }
    }
    }
    }
  }
}
