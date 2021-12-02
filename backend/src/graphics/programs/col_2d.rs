use super::super::common as cm;
use js_sys::WebAssembly;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as GL;
use web_sys::*;

pub struct Col2D {
    angle: f32,
    v_rect_len: usize,
    v_buffer: WebGlBuffer,
    u_col: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
    program: WebGlProgram,
}
impl Col2D {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = cm::link_program(
            &gl,
            super::super::shaders::vertex::col_2d::SHADER,
            super::super::shaders::fragment::col_2d::SHADER,
        )
        .unwrap();

        let v_rect: [f32; 12] = [

            -1.00125, 0.00125,
            -1.00125, -0.00125,
            1.00125, 0.00125,

             1.00125, 0.00125,
            -1.00125, -0.00125,
             1.00125, -0.00125,
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
        let anti_aspect = cm::scale4(1.,dxdy,1.);

        let full_transform = cm::mult_matrix_4(cm::mult_matrix_4(cm::rotate_z_4(self.angle), cm::identity4()), anti_aspect);

        self.angle+=0.01;

        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &full_transform );
        gl.draw_arrays(GL::TRIANGLES, 0, (self.v_rect_len / 2) as i32);
    }
}

pub struct Circle2D {
    angle: f32,
    v_triangles_len: usize,
    v_buffer: WebGlBuffer,
    u_col: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
    program: WebGlProgram,
}

impl Circle2D {
    pub fn new(gl: &WebGlRenderingContext) -> Self {
        let program = cm::link_program(
            &gl,
            super::super::shaders::vertex::col_2d::SHADER,
            super::super::shaders::fragment::col_2d::SHADER,
        )
        .unwrap();

        // calc number of triangles from definition
        let angle : f32 = 0.0174533; // 1 degree
        let mut v_circle = [0.; 3600*6];

        for i in 0..(3599) {
            v_circle[(i*6) as usize] = 0.; //centre x
            v_circle[((i*6) + 1) as usize] = 0.; //centre y

            v_circle[((i*6) + 2) as usize] = -1. * (i as f32 * angle).sin(); // x
            v_circle[((i*6) + 3) as usize] = (i as f32 * angle).cos(); // y

            v_circle[((i*6) + 4) as usize] = -1. * ((i+1) as f32 * angle).sin(); // x
            v_circle[((i*6) + 5) as usize] = ((i+1) as f32 * angle).cos(); // y
        }

        let mem_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()
            .unwrap()
            .buffer();

        let v_location = v_circle.as_ptr() as u32 / 4;
        let v_array = js_sys::Float32Array::new(&mem_buffer)
            .subarray(v_location, v_location + v_circle.len() as u32);

        let buffer_circle = gl.create_buffer().ok_or("failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer_circle));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &v_array, GL::STATIC_DRAW);

        Self {
            angle: 0.,
            v_triangles_len: v_circle.len(),
            u_col: gl.get_uniform_location(&program, "uColor").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
            v_buffer: buffer_circle,
            program: program,
        }
    }

    pub fn render(
        &mut self,
        gl: &WebGlRenderingContext,
        height: f32,
        width: f32,
    ) {
        gl.use_program(Some(&self.program));
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.v_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);
        gl.uniform1f(Some(&self.u_opacity),1.);

        // undo aspect ratio scaling
        let dxdy = width / height;
        let anti_aspect = cm::scale4(1.,dxdy,1.);

        self.angle += 0.01;

        for i in 0..20{

            for j in 0..20{

                gl.uniform4f(Some(&self.u_col),
                 (self.angle + i as f32).sin(),
                 (self.angle + j as f32).cos(),
                 1.,
                 1.0);

                let transform = cm::mult_matrix_4(
                cm::scale4(0.01,0.01,0.01),
                cm::mult_matrix_4(cm::rotate_y_4(0.),
                cm::translate4(-0.95 + 0.1*j as f32, -0.95 + 0.1*i as f32,0.)));

                gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false,  &cm::mult_matrix_4(anti_aspect, transform) );
                gl.draw_arrays(GL::TRIANGLES, 0, (self.v_triangles_len / 2) as i32);
            }
        }
    }
}
