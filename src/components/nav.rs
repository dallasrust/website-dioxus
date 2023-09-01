use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn Nav(cx: Scope) -> Element {
  render! {
    div {
      class: "box",
    span {
      class: "drum",
      "Dallas Rust User Meetup",
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
}
