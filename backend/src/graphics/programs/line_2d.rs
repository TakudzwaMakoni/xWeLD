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
pub struct Col2D {
    angle: f32,
    v_rect_len: usize,
    v_buffer: WebGlBuffer,
    u_col: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
    program: WebGlProgram,
}

#[allow(dead_code)]
impl Col2D {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = setup::link_program(
            &gl,
            super::super::shaders::vertex::col_2d::SHADER,
            super::super::shaders::fragment::col_2d::SHADER,
        )
        .unwrap();

        let v_rect: [f32; 12] = [

            0., 0.00125,
            0., -0.00125,
            1., 0.00125,

            1., 0.00125,
            0., -0.00125,
            1., -0.00125,
        ];

        let mem_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let v_location = v_rect.as_ptr() as u32 / 4;
        let v_array = js_sys::Float32Array::new(&mem_buffer)
            .subarray(v_location, v_location + v_rect.len() as u32);

        let buffer_rect = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_rect));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &v_array, GL::STATIC_DRAW);

        Self {
            angle: 0.,
            v_rect_len: v_rect.len(),
            u_col: gl.get_uniform_location(&program, "uColor").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
            v_buffer: buffer_rect,
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
        gl.use_program(Some(&self.program));
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.v_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform4f(Some(&self.u_col),
         self.angle.sin(),
         self.angle.cos(),
         1.,
         1.0);
        gl.uniform1f(Some(&self.u_opacity),1.);
        // undo aspect ratio scaling
        let dxdy = width / height;
        let anti_aspect = mat::scale4(1.,dxdy,1.);

        // do data transform
        for node in data.iter() {
            for force in node.forces.iter() {

                gl.uniform4f(Some(&self.u_col),
                    1.,
                    0.,
                    0.,
                    1.0);
                gl.uniform1f(Some(&self.u_opacity),1.);

               // undo aspect ratio scaling
                let dxdy = width / height;
                let anti_aspect = mat::scale4(1.,dxdy,1.);
                let neighbour = &data[force.indices[0]];
                let pos1 = [node.position[0],node.position[1], node.position[2],];
                let pos2 = [neighbour.position[0], neighbour.position[1], neighbour.position[2],];
                let ab = vec::sub(pos1, pos2);
                let len = vec::norm(vec::sub(pos1, pos2));

                // angle from x-axis
                self.angle = vec::angle([1.,0.,0.], ab);

                let translation = mat::translate4(node.position[0], node.position[1], node.position[2]);
                let scale = mat::scale4(len, 1., 1.);
                let rotation = mat::rotate_z_4(self.angle);
                let transform = mat::mult_matrix_4(anti_aspect, mat::mult_matrix_4(scale, rotation));
                let full_transform = mat::mult_matrix_4(translation, transform);

                gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &full_transform );
                gl.draw_arrays(GL::TRIANGLES, 0, (self.v_rect_len / 2) as i32);
        
            }
        }
    }
}
