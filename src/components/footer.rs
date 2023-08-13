use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Footer(cx: Scope) -> Element {
  render! {
    p {
      "Copyright 2023 "
      a {
        href: "https://github.com/dallasrust/website-dioxus",
        target: "_blank",
        "Dallas Rust Users Meetup Contributors"
      }
    }
  }
}
