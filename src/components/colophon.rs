use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Colophon(cx: Scope) -> Element {
  render! {
    div {
      class: "box",
    h1 { "Colophon" }
    ul {
    p {
      "Created using "
      a { href: "https://dioxuslabs.com/", target: "_blank", "Dioxus" }
    }
    p {
      "Open source repositories for this site:"
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
}
