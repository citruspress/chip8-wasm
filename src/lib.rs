mod chip8;
mod webgl;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let renderer = chip8::Renderer::new()?;
    let mut screen = chip8::Screen::new();

    renderer.render(&screen);

    Ok(())
}
