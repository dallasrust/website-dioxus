use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn About(cx: Scope) -> Element {
  render! {
    h1 {
      "Dallas Rust Users Meetup / About"
    }
    h2 {
      "Contact"
    }
    p {
      "David Wallace Croft <david@CroftSoft.com>"
    }
  }
}
