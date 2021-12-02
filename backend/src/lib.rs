extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

mod graphics {
    pub mod common;
    pub mod programs;
    pub mod setup;
    pub mod shaders;
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub struct Client {
    gl: WebGlRenderingContext,
    program_col_2d: graphics::programs::Col2D,
    program_circle_2d: graphics::programs::Circle2D,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl_ = graphics::setup::initialise_webgl_context().unwrap();
        Self {
            program_col_2d: graphics::programs::Col2D::new(&gl_),
            program_circle_2d: graphics::programs::Circle2D::new(&gl_),
            gl: gl_,
        }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        Ok(())
    }

    pub fn draw(&mut self, _height: f32, _width: f32) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        self.program_col_2d
                .render(&mut self.gl, _height, _width);
        self.program_circle_2d
            .render(&mut self.gl, _height, _width);

    }
}
