#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub mod data;
pub mod item;
pub mod math;
pub mod simulator;
pub mod time;
pub mod view;

pub use app::App;
pub use view::View;

pub mod prelude {
    pub use crate::item::{
        draw_cart, Circle, Ellipse, Rectangle, Shape, WithAngle, WithPosition, WithSize,
    };
    pub use crate::math::{cos, sin};
    pub use crate::time::Timer;
    pub use crate::View;
}

// ----------------------------------------------------------------------------
// When compiling for web:

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    eframe::start_web(canvas_id, Box::new(|cc| Box::new(App::new(cc))))
}
