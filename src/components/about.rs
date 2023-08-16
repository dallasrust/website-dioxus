use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn About(cx: Scope) -> Element {
  render! {
    div {
      class: "box",
    h1 {
      "About"
    }
    p {
      "The Dallas Rust Users Meetup (DRUM) is a ",
      a {
        href: "https://www.rust-lang.org/",
        target: "_blank",
        "Rust programming language",
      }
      " users group. ",
    }
    p {
      "Join our ",
      a {
        href: "https://www.meetup.com/dallasrust/",
        target: "_blank",
        "Meetup group",
      },
      " for an invitations to join our Discord meetings ",
      "and to access our bulletin boards."
    }
    h2 {
      "Contact"
    }
    p {
      "David Wallace Croft ",
      a {
        href: "mailto:david@croftsoft.com",
        "<david@CroftSoft.com>"
      }
    }
    }
  }
}
