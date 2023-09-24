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
    p {
      span {
        font_weight: "bold",
        "Employers and Recruiters"
      }
      ": If you have a Rust job based in the Dallas area, "
      "we invite you to post the job description "
      "on our Meetup website message board forum "
      a {
        href: "https://www.meetup.com/dallasrust/messages/boards/forum/38878463",
        target: "_blank",
        "Dallas Rust Jobs"
      }
      "."
    }
    p {
      span {
        font_weight: "bold",
        "Job Seekers"
      }
      ": If you are looking for a Rust job in the Dallas area, "
      "please feel free to post your résumé "
      "on our Meetup website message board forum "
      a {
        href: "https://www.meetup.com/dallasrust/messages/boards/forum/38878467",
        target: "_blank",
        "Dallas Rust Résumés",
      }
      "."
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
