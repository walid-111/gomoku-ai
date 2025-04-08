pub use wasm_bindgen::prelude::*;

pub fn set_panic_hook() {
	// When the `console_error_panic_hook` feature is enabled, we can call the
	// `set_panic_hook` function at least once during initialization which leads to better error messages in the browser console.
	// https://github.com/rustwasm/console_error_panic_hook#readme
	#[cfg(feature = "console_error_panic_hook")]
	console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct DebugOverlayCell {
	#[wasm_bindgen]
	pub value: Option<f32>,
}

#[wasm_bindgen]
pub struct DebugOverlaySettings {
	#[wasm_bindgen]
	pub max: f32,

	#[wasm_bindgen]
	pub precision: usize,
}

#[macro_export]
macro_rules! debug_log {
	($($arg:tt)*) => {{
		_debug_log(&format!($($arg)*));
	}};
}

#[macro_export]
macro_rules! debug_time {
	($label:literal, $code:block) => {{
		_debug_time_start($label);
		let result = $code;
		_debug_time_end($label);
		result
	}};
}

pub use js_ffi::*;

#[cfg(target_arch = "wasm32")]
mod js_ffi {
	use super::*;

	#[wasm_bindgen]
	extern "C" {
		#[wasm_bindgen(js_name = debugLog)]
		pub fn _debug_log(message: &str);

		#[wasm_bindgen(js_name = debugTimeStart)]
		pub fn _debug_time_start(label: &str);

		#[wasm_bindgen(js_name = debugTimeEnd)]
		pub fn _debug_time_end(label: &str);

		#[wasm_bindgen(js_name = debugOverlay)]
		pub fn _debug_overlay(name: &str,
		                     settings: DebugOverlaySettings,
		                     values: Vec<DebugOverlayCell>);
	}
}

#[cfg(not(target_arch = "wasm32"))]
mod js_ffi {
	use super::*;

	pub fn _debug_log(message: &str) {
		eprintln!("{message}");
	}
	pub const fn _debug_time_start(_label: &str) {}
	pub const fn _debug_time_end(_label: &str) {}
	pub fn _debug_overlay(_name: &str,
	                     settings: DebugOverlaySettings,
	                     values: Vec<DebugOverlayCell>) {
		_ = (settings, values);
	}
}

pub fn debug_overlay(name: &str, settings: DebugOverlaySettings, values: Vec<Option<f32>>) {
	_debug_overlay(name, settings, values.into_iter().map(|value| DebugOverlayCell { value }).collect());
}
