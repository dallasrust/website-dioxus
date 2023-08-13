use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn NavBar(cx: Scope) -> Element {
  render! {
    Outlet::<Route> {}
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
