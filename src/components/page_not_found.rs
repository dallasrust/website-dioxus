use dioxus::prelude::*;

#[allow(non_snake_case)]
#[inline_props]
pub fn PageNotFound(
  cx: Scope,
  route: Vec<String>,
) -> Element {
  render! {
    h1 {
      "Page Not Found"
    }
    pre {
      color: "red",
      "{route:?}"
    }
  }
}
