use super::super::route::Route;
use super::footer::Footer;
use super::nav::Nav;
use ::dioxus::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Template() -> Element {
  static CSS: Asset = asset!("/assets/template/stylesheet.css");

  static FAVICON: Asset = asset!("/assets/template/favicon.ico");

  rsx! {
    document::Stylesheet {
      href: CSS
    }
    document::Link {
      href: FAVICON,
      rel: "icon",
    }
    div {
      class: "frame",
    div {
      class: "col-1",
    Nav { }
    }
    div {
      class: "col-2",
      Outlet::<Route> {}
    }
    div {
      class: "col-3",
    Footer { }
    }
    }
  }
}
