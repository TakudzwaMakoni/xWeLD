extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

mod common;
mod graphics {
    pub mod programs;
    pub mod setup;
    pub mod shaders;
}

mod physics {
    pub mod lattice;
    pub mod verlet;
    pub mod harmonic {
        pub mod spring;
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}


#[wasm_bindgen]
#[allow(dead_code)]
pub struct Client {
    data:Vec<physics::lattice::Node>,
    updates: Vec<fn(data: &mut Vec<physics::lattice::Node>)>,
    gl: WebGlRenderingContext,
    program_col_2d: graphics::programs::Line2D,
    program_circle_2d: graphics::programs::Circle2D,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        let gl_ = graphics::setup::initialise_webgl_context().unwrap();

        // generate data
        let mut data = physics::lattice::primitive_cubic(2,2,2,0.1);
        physics::harmonic::spring::generate_spring_forces(&mut data, 1.8, 0.1);
        // data[1].position[0] = 0.05;

        Self {
            data: data,
            updates: vec![physics::verlet::resolve_forces, physics::verlet::velocity_verlet, physics::verlet::update_state],
            program_col_2d: graphics::programs::Line2D::new(&gl_),
            program_circle_2d: graphics::programs::Circle2D::new(&gl_),
            gl: gl_,
        }
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        for u in self.updates.iter(){
            u(&mut self.data);
        }
        Ok(())
    }

    pub fn draw(&mut self, _height: f32, _width: f32) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        self.program_col_2d
                .render(&mut self.gl, &self.data, _height, _width); 
        self.program_circle_2d
            .render(&mut self.gl, &self.data, _height, _width);

    }
}
