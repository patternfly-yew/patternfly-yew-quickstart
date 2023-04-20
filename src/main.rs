#![recursion_limit = "1024"]

mod app;
mod components;
mod counter;
mod example;
mod full;
mod hook;
mod icons;
mod index;
mod layouts;

use wasm_bindgen::prelude::*;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: log::Level = log::Level::Info;
#[cfg(debug_assertions)]
const LOG_LEVEL: log::Level = log::Level::Trace;

pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(LOG_LEVEL));
    yew::Renderer::<app::Application>::new().render();
    Ok(())
}
