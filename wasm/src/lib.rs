use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    alert(&format!("Hello!"));
    Ok(())
}
