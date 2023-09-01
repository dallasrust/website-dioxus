use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Footer(cx: Scope) -> Element {
  render! {
    div {
      class: "box",
    p {
      dangerous_inner_html: "&copy;",
      " 2023 "
      a {
        href: "https://github.com/dallasrust/website-dioxus",
        target: "_blank",
        "Dallas Rust User Meetup Contributors"
      }
    }
  }
  }
}
