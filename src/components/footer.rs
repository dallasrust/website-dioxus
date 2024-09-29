use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Footer(cx: Scope) -> Element {
  render! {
    div {
      class: "box",
      text_align: "center",
    p {
      dangerous_inner_html: "&copy;",
      " 2023-2024 "
      a {
        href: "https://github.com/dallasrust/website-dioxus",
        target: "_blank",
        "DRUM Contributors"
      }
    }
    img {
      src: "/corro.svg",
      width: "100",
    }
  }
  }
}
