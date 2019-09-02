mod chip8;
mod webgl;

use std::cell::RefCell;
use wasm_bindgen::prelude::*;

struct Data {
    pub renderer: chip8::Renderer,
    pub screen: chip8::Screen,
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    thread_local! {
        static DATA: RefCell<Data> = RefCell::new(Data {
            renderer: chip8::Renderer::new().expect("failed to initialize renderer"),
            screen: chip8::Screen::new(),
        });
    }

    DATA.with(|data| {
        let mut data = data.borrow_mut();
        data.screen.clear();
        data.screen.draw_sprite(5, 5, &vec![0xff, 0xff, 0xff]);

        data.renderer.render(&data.screen);
    });

    Ok(())
}
