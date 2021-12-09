
pub mod io {
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        pub fn log(s: &str);
    }
}

pub mod linear_algebra {
    pub mod matrix {
        pub fn translate4(tx: f32, ty: f32, tz: f32) -> [f32; 16] {
            let mut arr = [0.; 16];

            arr[0] = 1.;
            arr[5] = 1.;
            arr[10] = 1.;
            arr[15] = 1.;

            arr[12] = tx;
            arr[13] = ty;
            arr[14] = tz;

            arr
        }

        #[allow(dead_code)]
        pub fn identity4() -> [f32; 16] {
            let mut arr = [0.; 16];

            arr[0] = 1.;
            arr[5] = 1.;
            arr[10] = 1.;
            arr[15] = 1.;

            arr
        }

        #[allow(dead_code)]
        pub fn scale4(sx: f32, sy: f32, sz: f32) -> [f32; 16] {
            let mut arr = [0.; 16];
            arr[0] = sx;
            arr[5] = sy;
            arr[10] = sz;
            arr[15] = 1.;

            arr
        }

        #[allow(dead_code)]
        pub fn rotate_x_4(angle: f32) -> [f32;16] {
            let mut arr = [0.;16];
            arr[0] = 1.;
            arr[5] = angle.cos();
            arr[6] = angle.sin();
            arr[9] = -1. * angle.sin();
            arr[10] = angle.cos();
            arr[15] = 1.;

            arr
        }

        #[allow(dead_code)]
        pub fn rotate_y_4(angle: f32) -> [f32;16] {
            let mut arr = [0.;16];
            arr[0] = angle.cos();
            arr[5] = 1.;
            arr[8] = angle.sin();
            arr[2] = -1. * angle.sin();
            arr[10] = angle.cos();
            arr[15] = 1.;

            arr
        }

        #[allow(dead_code)]
        pub fn rotate_z_4(angle: f32) -> [f32;16] {
            let mut arr = [0.;16];
            arr[0] = angle.cos();
            arr[5] = angle.cos();
            arr[1] = angle.sin();
            arr[4] = -1. * angle.sin();
            arr[10] = 1.;
            arr[15] = 1.;

            arr
        }

        pub fn mult_matrix_4(a: [f32; 16], b: [f32; 16]) -> [f32; 16] {
            let mut return_var = [0.; 16];

            return_var[0] = a[0] * b[0] + a[1] * b[4] + a[2] * b[8] + a[3] * b[12];
            return_var[1] = a[0] * b[1] + a[1] * b[5] + a[2] * b[9] + a[3] * b[13];
            return_var[2] = a[0] * b[2] + a[1] * b[6] + a[2] * b[10] + a[3] * b[14];
            return_var[3] = a[0] * b[3] + a[1] * b[7] + a[2] * b[11] + a[3] * b[15];

            return_var[4] = a[4] * b[0] + a[5] * b[4] + a[6] * b[8] + a[7] * b[12];
            return_var[5] = a[4] * b[1] + a[5] * b[5] + a[6] * b[9] + a[7] * b[13];
            return_var[6] = a[4] * b[2] + a[5] * b[6] + a[6] * b[10] + a[7] * b[14];
            return_var[7] = a[4] * b[3] + a[5] * b[7] + a[6] * b[11] + a[7] * b[15];

            return_var[8] = a[8] * b[0] + a[9] * b[4] + a[10] * b[8] + a[11] * b[12];
            return_var[9] = a[8] * b[1] + a[9] * b[5] + a[10] * b[9] + a[11] * b[13];
            return_var[10] = a[8] * b[2] + a[9] * b[6] + a[10] * b[10] + a[11] * b[14];
            return_var[11] = a[8] * b[3] + a[9] * b[7] + a[10] * b[11] + a[11] * b[15];

            return_var[12] = a[12] * b[0] + a[13] * b[4] + a[14] * b[8] + a[15] * b[12];
            return_var[13] = a[12] * b[1] + a[13] * b[5] + a[14] * b[9] + a[15] * b[13];
            return_var[14] = a[12] * b[2] + a[13] * b[6] + a[14] * b[10] + a[15] * b[14];
            return_var[15] = a[12] * b[3] + a[13] * b[7] + a[14] * b[11] + a[15] * b[15];

            return_var
        }
    }

    pub mod vector {
        pub fn add(vec1: [f32;3], vec2: [f32;3]) -> [f32;3] {
            let mut new_vec = [0.;3];
            new_vec[0] = vec1[0] + vec2[0];
            new_vec[1] = vec1[1] + vec2[1];
            new_vec[2] = vec1[2] + vec2[2];
            new_vec
        }

        pub fn sub(vec1: [f32;3], vec2: [f32;3]) -> [f32;3] {
            let mut new_vec = [0.;3];
            new_vec[0] = vec1[0] - vec2[0];
            new_vec[1] = vec1[1] - vec2[1];
            new_vec[2] = vec1[2] - vec2[2];
            new_vec
        }

        pub fn dot(vec1: [f32;3], vec2: [f32;3]) -> f32 {
            let mut dot_product : f32 = 0.;
            dot_product += vec1[0] * vec2[0];
            dot_product += vec1[1] * vec2[1];
            dot_product += vec1[2] * vec2[2];
            dot_product
        }

        pub fn scale(scalar: f32, v : [f32;3]) -> [f32;3] {
            [scalar * v[0], scalar * v[1], scalar * v[2]]
          }

        pub fn norm( v : [f32;3]) -> f32 {
            let dot_product = dot(v, v);
            dot_product.sqrt()
        }

        pub fn angle(v1 : [f32;3], v2 : [f32;3]) -> f32 {
            let denom =  norm(v1) * norm(v2);
            assert!(denom!=0.);
            let num = dot(v1,v2);
            (num / denom).acos()
          }

        pub fn unit_v(v: [f32;3]) -> [f32;3] {
          let n = norm(v);
          let u = scale(1./n, v);
          u
        }

        #[cfg(test)]
        mod tests {
            // Note this useful idiom: importing names from outer (for mod tests) scope.
            use super::*;

            fn approx_eq(a:f32, b:f32, precision: f32) -> bool {
                (a - b).abs() < precision
            }

            #[test]
            fn test_add() {
                let v1 = [1.,2.,3.];
                let v2 = [4.,5.,6.];
                let v_test = add(v1,v2);
                let v_result = [5.,7.,9.];
                assert_eq!(v_test.len(), v_result.len(), "Arrays don't have the same length");
                assert!(v_test.iter().zip(v_result.iter()).all(|(a,b)| a == b), "Arrays are not equal");
            }

            #[test]
            fn test_sub() {
                let v1 = [1.,2.,3.];
                let v2 = [4.,5.,6.];
                let v_test = sub(v1,v2);
                let v_result = [-3.,-3.,-3.];
                assert_eq!(v_test.len(), v_result.len(), "Arrays don't have the same length");
                assert!(v_test.iter().zip(v_result.iter()).all(|(a,b)| a == b), "Arrays are not equal");
            }

            #[test]
            fn test_dot() {
                let v1 = [1.,2.,3.];
                let v2 = [4.,5.,6.];
                let test = dot(v1,v2);
                let result = 32.;
                assert_eq!(test, result);
            }

            #[test]
            fn test_norm() {
                let v = [1.,2.,2.];
                let test = norm(v);
                let result = 3.;
                assert_eq!(test, result);
            }

            #[test]
            fn test_scale() {
                let v = [1.,2.,3.];
                let v_test = scale(2.,v);
                let v_result = [2.,4.,6.,];
                assert_eq!(v_test.len(), v_result.len(), "Arrays don't have the same length");
                assert!(v_test.iter().zip(v_result.iter()).all(|(a,b)| a == b), "Arrays are not equal");
            }

            #[test]
            fn test_angle() {
                let v1 = [1.,0.,0.];
                let v2 = [0.,1.,0.];
                let v3 = [-1.,0.,0.];
                assert!(approx_eq(angle(v1,v2), 0.5*std::f32::consts::PI, 0.0005));
                assert!(approx_eq(angle(v1,v3), std::f32::consts::PI, 0.0005));
                assert!(approx_eq(angle(v3,v2), 0.5*std::f32::consts::PI, 0.0005));
            }

            #[test]
            fn test_unit_vec() {
                let v = [11.3,0.053,400.];
                let u = unit_v(v);
                assert_eq!(norm(u), 1.);
                assert_eq!(angle(u,v),0.);
            }
        }
    }

}
