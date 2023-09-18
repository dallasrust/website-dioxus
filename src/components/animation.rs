use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Animation(cx: Scope) -> Element {
  let scale_x_state: &UseState<i8> = use_state(cx, || 1);
  render! {
    img {
      cursor: "pointer",
      onclick: move |_event| transform(scale_x_state),
      onmouseenter: move |_event| transform(scale_x_state),
      onmouseout: move |_event| transform(scale_x_state),
      onwheel: move |_event| transform(scale_x_state),
      src: "/rustacean-flat-gesture.svg",
      transform: "scaleX({scale_x_state})",
    }
  }
}

fn transform(mut scale_x_state: &UseState<i8>) {
  scale_x_state *= -1;
}
