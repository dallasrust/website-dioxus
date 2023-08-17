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
      " to access our bulletin boards"
      " and receive invitations to our periodic Discord meetings.",
    }
    h2 {
      "Contact"
    }
    p {
      a {
        href: "https://www.croftsoft.com/people/david/",
        target: "_blank",
        "David Wallace Croft",
      },
      br { },
      "DRUM Organizer",
      br { },
      a {
        href: "mailto:david@croftsoft.com",
        "david@CroftSoft.com"
      },
      br { },
      "(214) 636-3790"
    }
    }
  }
}
