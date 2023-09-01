use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::Link;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
  render! {
    div {
      class: "app-home box",
    h1 {
      "Dallas Rust User Meetup"
    }
    a {
      href: "https://www.meetup.com/dallasrust/",
      target: "_blank",
      "https://www.meetup.com/dallasrust/"
    }
    br { }
    a {
      href: "https://www.meetup.com/dallasrust/",
      target: "_blank",
    img {
      src: "rustacean-flat-happy.svg",
    }
    }
    }
    div {
      class: "box",
      style: "margin-top: 2rem",
    span {
      style: "font-weight: bold",
    "2023 Aug 17 Thu"
    },
    p {
      style: "margin-top: 1rem",
    "The Dallas Rust User Meetup (DRUM) is seeking a location within",
    " the Dallas area for our next face-to-face meeting."
    " We are looking for a site with a laptop projector."
    " If you would like to host the DRUM, please contact "
    Link { to: Route::About {}, "us" },
    "."
    }
    }
  }
}
