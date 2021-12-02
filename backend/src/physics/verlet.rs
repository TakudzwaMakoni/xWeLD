use super::lattice::*;
use super::vector::*;
use super::super::graphics::common::log;
use std::collections::HashMap;



fn test_force(node_index: usize, force_index: usize, data: &Vec<Node>) -> Vec<(usize,[f32;3])> {
    vec![(0,[0.5,0.,0.])]
}

fn spring_force(node_index: usize, force_index: usize, data: &Vec<Node>) -> Vec<(usize,[f32;3])> {

    let k = data[node_index].forces[force_index].params[0];
    let nodes_len = data[node_index].forces[force_index].params[1];
    let neighbour_index = data[node_index].forces[force_index].indices[0];

    let d1 = &data[node_index];
    let d2 = &data[neighbour_index];

    let dx = d1.position[0] - d2.position[0];
    let dy = d1.position[1] - d2.position[1];
    let dz = d1.position[2] - d2.position[2];

    let separation = [dx, dy, dz];
    let unit_separation = unit_v(separation);

    let equilibrium = scale(nodes_len, unit_separation);
    let extension = sub(separation, equilibrium);

    let fx = -k * extension[0];
    let fy = -k * extension[1];
    let fz = -k * extension[2];

    vec![(node_index,[fx,fy,fz])]
}

pub struct ForceLibrary {
    //map: HashMap<&'a str, fn(node:Node, data: Vec<Node>, params: [f32;3]) -> Vec<[f32;3]>>,
    spring: fn(node_index: usize, force_index: usize, data: &Vec<Node>) -> Vec<(usize,[f32;3])>,
    //valence_angle: fn(node:Node, data: Vec<Node>, params: [f32;3]) -> Vec<[f32;3]>,
    test: fn(node_index: usize, force_index: usize, data: &Vec<Node>) -> Vec<(usize,[f32;3])>,
}

impl ForceLibrary {
    pub fn new() -> Self {
        Self{
            spring: spring_force,
            test: test_force,
        }
    }
}

pub fn resolve_forces(data: &mut Vec<Node>){
    let force_lib = ForceLibrary::new();
    for i in 0..data.len() {
        for j in 0..(data[i].forces).len() {
            let actions = (force_lib.test)(i,j,&data);
            for k in 0..actions.len() {
                let action = actions[k];
                let index = action.0;
                let force = action.1;
                let node = &mut (data[index]);
                node.net_force[0] += force[0];
                node.net_force[1] += force[1];
                node.net_force[2] += force[2];
            }
        }
    }
}

pub fn velocity_verlet(data: &mut Vec<Node>){

    //for force in node.forces.iter() {
    //    log(&format!("{:?}", force.params));
    //}
    //for force in node.forces.iter() {
    //    let actions = (force_lib.test)(index, &data, &force);
    //    for action in actions.iter() {
    //        (data[index]).position[0] += action[0];
    //    }
    //}
}
