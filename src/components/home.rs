use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
  render! {
      h1 { "Dallas Rust Users Meetup" }
      a { href: "https://www.meetup.com/dallasrust/", target: "_blank", "https://www.meetup.com/dallasrust/" }
  }
}
