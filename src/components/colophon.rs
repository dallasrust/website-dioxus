use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Colophon(cx: Scope) -> Element {
  render! {
    div {
      class: "app-colophon box",
    h1 { "Colophon" }
    ul {
    p {
      "This website was created using the Rust-based user interface library ",
      a {
        href: "https://dioxuslabs.com/",
        target: "_blank",
        "Dioxus",
      },
      "."
    }
    p {
      "The images of Ferris the Rustacean and "
      "Corro the Unsafe Rusturchin are from "
      a {
        href: "https://www.rustacean.net/",
        target: "_blank",
        "Rustacean.net",
      },
      "."
    }
    p {
      "The open source repositories for this website are hosted on GitHub:"
    }
    ul {
    li {
    a {
      href: "https://github.com/dallasrust/website-dioxus",
      target: "_blank",
      "https://github.com/dallasrust/website-dioxus"
    }
    }
    li {
    a {
      href: "https://github.com/dallasrust/dallasrust.github.io",
      target: "_blank",
      "https://github.com/dallasrust/dallasrust.github.io"
    }
    }
    }
    }
    }
  }
}
