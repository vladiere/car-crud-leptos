pub mod components;
pub mod icons;
pub mod layouts;
pub mod models;
pub mod servers;
pub mod utils;
pub mod views;

pub mod app;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
