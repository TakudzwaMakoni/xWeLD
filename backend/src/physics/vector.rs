
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
    let mut dot_product = dot(v, v);
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
