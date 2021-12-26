use crate::common::linear_algebra::matrix as mat;
use crate::common::linear_algebra::vector as vec;

use crate::common::io::log;
use super::super::setup as setup;
use super::super::super::physics as ph;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

#[allow(dead_code)]
pub struct Line2D {
    angle: f32,
    u_col: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
    program: WebGlProgram,
}

#[allow(dead_code)]
impl Line2D {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = setup::link_program(
            &gl,
            super::super::shaders::vertex::col_2d::SHADER,
            super::super::shaders::fragment::col_2d::SHADER,
        )
        .unwrap();

        Self {
            angle: 0.,
            u_col: gl.get_uniform_location(&program, "uColor").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
            program: program,
        }
    }

    pub fn render(
        &mut self,
        gl: &WebGlRenderingContext,
        data: &Vec<ph::lattice::Node>,
        height: f32,
        width: f32,
    ) {


        // undo aspect ratio scaling
        let dxdy = width / height;
        let anti_aspect = mat::scale4(1.,dxdy,1.);
    
        // do data transform
        for node in data.iter() {

            for force in node.forces.iter() {

                gl.uniform4f(Some(&self.u_col),
                    1.,
                    1.,
                    0.,
                    1.0);
                gl.uniform1f(Some(&self.u_opacity),1.);

               // undo aspect ratio scaling
                let dxdy = width / height;
                let anti_aspect = mat::scale4(1.,dxdy,1.);
                let neighbour = &data[force.indices[0]];
                let pos1 = [node.position[0],node.position[1], node.position[2],];
                let pos2 = [neighbour.position[0], neighbour.position[1], neighbour.position[2],];

                let v_rect: [f32; 6] = [
                    pos1[0], pos1[1], pos1[2],
                    pos2[0], pos2[1], pos2[2],
                ];

                let mem_buffer = wasm_bindgen::memory()
                    .dyn_into::<WebAssembly::Memory>()
                    .unwrap()
                    .buffer();

                let v_location = v_rect.as_ptr() as u32 / 4;
                let v_array = js_sys::Float32Array::new(&mem_buffer)
                    .subarray(v_location, v_location + v_rect.len() as u32);

                let v_buffer = gl.create_buffer().ok_or("failed to create buffer").unwrap();
                gl.bind_buffer(GL::ARRAY_BUFFER, Some(&v_buffer));
                gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &v_array, GL::STATIC_DRAW);

                gl.use_program(Some(&self.program));
                gl.bind_buffer(GL::ARRAY_BUFFER, Some(&v_buffer));
                gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false,3 * std::mem::size_of::<f32>() as i32, 0);
                gl.enable_vertex_attrib_array(0);


                let translation = mat::translate4(node.position[0], node.position[1], node.position[2]);
                let scale = mat::mult_matrix_4(mat::scale4(1., 0.3, 1.), anti_aspect);
                let rotation = mat::rotate_z_4(self.angle);
                let rotate_and_translate = mat::mult_matrix_4(rotation, translation);
                let full_transform = mat::mult_matrix_4(scale, rotate_and_translate);
                
                gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &mat::identity4());
                gl.draw_arrays(GL::LINES, 0, 2);
        
            }
        }
    }
}
