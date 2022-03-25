#![recursion_limit = "1024"]

mod app;
mod components;
mod counter;
mod example;
mod full;
mod index;
mod layouts;

use log::Level;
use wasm_bindgen::prelude::*;

pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(Level::Debug));
    yew::start_app::<app::Model>();
    Ok(())
}
