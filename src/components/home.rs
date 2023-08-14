use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
  render! {
    div {
      class: "box",
    h1 {
      "Home"
    }
    a {
      href: "https://www.meetup.com/dallasrust/",
      target: "_blank",
      "https://www.meetup.com/dallasrust/"
    }
    }
  }
}
