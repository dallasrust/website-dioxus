use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Benefits() -> Element {
  rsx! {
    div {
      class: "box",
    h1 {
    "Benefits"
    }
    p {
      text_align: "justify",
    span {
      font_weight: "bold",
    "Dallas Rust User Meetup Members"
    }
    " (DRUMMers) can get a discount from Manning Publishing by using the "
    a {
      href: "https://www.manning.com/?utm_source=dallasrust&utm_medium=affiliate&utm_campaign=affiliate&a_aid=dallasrust",
      target: "_blank",
      "Dallas Rust affiliate link",
    }
    ".",
    }
    }
  }
}
