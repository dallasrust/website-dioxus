use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn About() -> Element {
  rsx! {
    div {
      class: "box",
    h1 {
      "About"
    }
    p {
      "The "
      span {
        font_weight: "bold",
        "Dallas Rust User Meetup"
      }
      " (DRUM) is a "
      a {
        href: "https://www.rust-lang.org/",
        target: "_blank",
        "Rust programming language",
      }
      " user group. ",
    }
    p {
      span {
        font_weight: "bold",
        "Join"
      }
      " our "
      a {
        href: "https://www.meetup.com/dallasrust/",
        target: "_blank",
        "Meetup group"
      },
      " to access our message boards"
      " and receive invitations to our meetings."
      " Anyone from anywhere is welcome to participate in our semimonthly "
      a {
        href: "https://discord.com/",
        target: "_blank",
        "Discord"
      }
      " server meetings."
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
