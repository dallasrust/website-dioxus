use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn About(cx: Scope) -> Element {
  render! {
    div {
      class: "box",
    h1 {
      "About"
    }
    h2 {
      "Contact"
    }
    p {
      "David Wallace Croft <david@CroftSoft.com>"
    }
    }
  }
}
