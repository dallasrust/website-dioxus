use dioxus::prelude::*;

#[allow(non_snake_case)]
#[inline_props]
pub fn PageNotFound(
  cx: Scope,
  route: Vec<String>,
) -> Element {
  render! {
      h1 { "Page not found" }
      p { "We are terribly sorry, but the page you requested doesn't exist." }
      pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
  }
}
