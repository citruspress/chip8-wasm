extern crate console_error_panic_hook;
extern crate rand;

mod chip8;
mod time;
mod webgl;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const CYCLES_PER_SECOND: u16 = 800;

struct Data {
    pub game_time: time::GameTime,
    pub cpu: chip8::Cpu,
    pub renderer: chip8::Renderer,
}

thread_local! {
    static DATA: RefCell<Data> = RefCell::new(Data {
        game_time: time::GameTime::new(now()),
        cpu: chip8::Cpu::new(),
        renderer: chip8::Renderer::new().expect("failed to initialize renderer"),
    });
}

#[wasm_bindgen]
pub fn load(rom: Vec<u8>) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    DATA.with(|data| {
        let mut data = data.borrow_mut();

        data.cpu.load_rom(&rom);
    });

    Ok(())
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn now() -> f64 {
    window()
        .performance()
        .expect("performance on window should be available")
        .now()
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        DATA.with(|data| {
            let mut data = data.borrow_mut();

            data.game_time.update(now());
            let steps = CYCLES_PER_SECOND as f64 * (data.game_time.elapsed() / 1000f64);

            for _ in 0..steps as u64 {
                data.cpu.step();
            }

            if data.cpu.screen.is_dirty() {
                data.renderer.render(&data.cpu.screen);
            }
        });

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
