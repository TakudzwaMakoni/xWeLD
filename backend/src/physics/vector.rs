
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

pub fn angle(v1 : [f32;3], v2 : [f32;3]){
    let denom =  norm(v1) * norm(v2);
    let num = dot(v1,v2);
    num / (if denom==0.{ 2.*std::f32::consts::PI } else { denom });
  }

pub fn unit_v(v: [f32;3]) -> [f32;3] {
  let n = norm(v);
  let u = scale(1./n, v);
  u
}
