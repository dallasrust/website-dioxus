use dioxus::prelude::*;
use dioxus_fullstack::{launch, prelude::*};
use dioxus_router::prelude::*;
use serde::{Deserialize, Serialize};
use website_dioxus::launch;
use website_dioxus::route::Route;

#[tokio::main]
async fn main() {
  pre_cache_static_routes_with_props(
    &ServeConfigBuilder::new_with_router(
      dioxus_fullstack::router::FullstackRouterConfig::<Route>::default(),
    )
    .assets_path("dist")
    .incremental(IncrementalRendererConfig::default().static_dir("dist"))
    .build(),
  )
  .await
  .unwrap();
}
