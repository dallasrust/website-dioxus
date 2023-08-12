use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[allow(non_snake_case)]
pub fn NavBar(cx: Scope) -> Element {
  render! {
      nav {
          ul {
              li {
                  Link { to: Route::Home {}, "Home" }
              }
              li {
                  Link { to: Route::About {}, "About" }
              }
          }
      }
      Outlet::<Route> {}
  }
}
