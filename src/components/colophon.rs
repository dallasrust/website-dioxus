use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Colophon(cx: Scope) -> Element {
  render! {
    div {
      class: "box",
    h1 { "Colophon" }
    ul {
    p {
      "This website was created using the Rust library ",
      a {
        href: "https://dioxuslabs.com/",
        target: "_blank",
        "Dioxus",
      },
      "."
    }
    p {
      "Images of Ferris the Crab and Corro the Unsafe Rusturchin are from ",
      a {
        href: "https://www.rustacean.net/",
        target: "_blank",
        "Rustacean.net",
      },
      "."
    }
    p {
      "The open source repository for this website is hosted on GitHub:",
    br { },
    a {
      href: "https://github.com/dallasrust/website-dioxus",
      target: "_blank",
      "https://github.com/dallasrust/website-dioxus"
    }
    }
    }
    }
  }
}
