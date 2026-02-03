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
mod panic;

use browser_panic_hook::{CustomBody, IntoPanicHook};
use wasm_bindgen::prelude::*;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: log::Level = log::Level::Info;
#[cfg(debug_assertions)]
const LOG_LEVEL: log::Level = log::Level::Trace;

pub fn main() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(LOG_LEVEL));
    yew::set_custom_panic_hook(
        CustomBody(Box::new(|details| {
            format!(
                r#"
<div class="pf-v6-l-bullseye">
  <div class="pf-v6-l-bullseye__item">
    <div class="pf-v6-c-alert pf-m-danger" aria-label="Application panicked">
      <div class="pf-v6-c-alert__icon">
        <i class="fas fa-fw fa-exclamation-circle" aria-hidden="true"></i>
      </div>
      <p class="pf-v6-c-alert__title">
        <span class="pf-v6-screen-reader">Panick alert:</span>
        Application panicked
      </p>
      <div class="pf-v6-c-alert__description">
        <p>The application failed critically and cannot recover.</p>
        <p>Reason: <pre>{message}</pre></p>
      </div>
    </div>
  </div>
</div>
"#,
                message = details.message()
            )
        }))
        .into_panic_hook(),
    );
    yew::Renderer::<app::Application>::new().render();
    Ok(())
}
