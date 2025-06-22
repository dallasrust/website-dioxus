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
    "Dallas Rust User Meetup Members (DRUMMers) can get 45% off of Manning "
    "books by using the discount code: "
    span {
      font_weight: "bold",
    "DallasRust45"
    }
    ".  Please use this affiliate link when you order from the "
    a {
      href: "https://mng.bz/gmY8",
      target: "_blank",
      "Manning website",
    }
    ".",
    }
    }
  }
}
