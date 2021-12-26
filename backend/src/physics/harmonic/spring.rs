use crate::physics::verlet::DT;
use super::super::lattice::*;
use crate::common::linear_algebra::vector as vec;

// a predicate for creating spring forces based on
// distance from neighbouring nodes
pub fn basic_spring_predicate(d1 : &Node, d2: &Node, equilibrium : f32) -> bool {
  if d1.id == d2.id { return false; }
  let pos1 = [d1.position[0], d1.position[1], d1.position[2]];
  let pos2 = [d2.position[0], d2.position[1], d2.position[2]];
  (vec::norm(vec::sub(pos1, pos2)) -  equilibrium).abs() < 0.01
}

pub fn generate_spring_forces(data : &mut Vec<Node>, spring_constant : f32, equilibrium : f32) {
    for i in 0..data.len() {
        for j in 0..data.len(){
            if basic_spring_predicate(&data[i], &data[j], equilibrium) {
                let id = data[j].id;
                data[i].forces.push(
                    Force {
                        name: String::from("spring"),
                        params: [ spring_constant, equilibrium, 0.],
                        indices: [id, 0],
                    }
                );
            }
        }
    }
}


pub fn spring_force(node_index: usize, force_index: usize, data: &Vec<Node>) -> Vec<(usize,[f32;6])> {

    let k = data[node_index].forces[force_index].params[0];
    let nodes_len = data[node_index].forces[force_index].params[1];
    let neighbour_index = data[node_index].forces[force_index].indices[0];

    let d1 = &data[node_index];
    let d2 = &data[neighbour_index];

    let dx = d1.position[0] - d2.position[0];
    let dy = d1.position[1] - d2.position[1];
    let dz = d1.position[2] - d2.position[2];

    let mut separation = [dx, dy, dz];
    let mut unit_separation = vec::unit_v(separation);

    let mut equilibrium = vec::scale(nodes_len, unit_separation);
    let mut extension = vec::sub(separation, equilibrium);

    let fx = -k * extension[0];
    let fy = -k * extension[1];
    let fz = -k * extension[2];

    // apply the force over time DT, calculate the extension again after DT,
    // then calculate the future force at DT. making it vv-algorithm compatible.

    // using position-verlet equation
    let d1_pos = [d1.position[0],d1.position[1],d1.position[2],];
    let d1_future_pos = vec::add(d1_pos, vec::scale(0.5*(1./d1.mass)*f32::powi(DT,2), [fx,fy,fz]));

    let dxf = d1_future_pos[0] - d2.position[0];
    let dyf = d1_future_pos[1] - d2.position[1];
    let dzf = d1_future_pos[2] - d2.position[2];

    separation = [dxf, dyf, dzf];
    unit_separation = vec::unit_v(separation);

    equilibrium = vec::scale(nodes_len, unit_separation);
    extension = vec::sub(separation, equilibrium);

    let fxf = -k * extension[0];
    let fyf = -k * extension[1];
    let fzf = -k * extension[2];

    vec![(node_index,[fx, fy, fz, fxf, fyf, fzf])]
}
