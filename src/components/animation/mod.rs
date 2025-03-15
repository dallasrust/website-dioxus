use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Animation() -> Element {
  static DRUMMER: Asset = asset!("/assets/animation/drummer.svg");

  let mut scale_x_signal: Signal<i8> = use_signal(|| 1);

  rsx! {
    img {
      cursor: "pointer",
      onclick: move |_event| transform(&mut scale_x_signal),
      onmouseenter: move |_event| transform(&mut scale_x_signal),
      onmouseout: move |_event| transform(&mut scale_x_signal),
      onwheel: move |_event| transform(&mut scale_x_signal),
      src: DRUMMER,
      transform: "scaleX({scale_x_signal})",
    }
  }
}

fn transform(scale_x_signal: &mut Signal<i8>) {
  scale_x_signal.set(-scale_x_signal());
}
