use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn render_svg(json_ir: &str) -> Result<String, JsValue> {
    console_error_panic_hook::set_once();
    archflow_core::render_svg(json_ir).map_err(|e| JsValue::from_str(&e.to_string()))
}
