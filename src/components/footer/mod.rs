use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Footer() -> Element {
  static CORRO: Asset = asset!("/assets/footer/corro.svg");

  rsx! {
    div {
      class: "box",
      text_align: "center",
    p {
      dangerous_inner_html: "&copy;",
      " 2023-2025 "
      a {
        href: "https://github.com/dallasrust/website-dioxus",
        target: "_blank",
        "DRUM Contributors"
      }
    }
    img {
      src: CORRO,
      width: "100",
    }
  }
  }
}
