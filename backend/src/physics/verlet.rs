use super::lattice::*;
use super::vector as vec;
use super::harmonic as harmonic;
use super::super::graphics::common::log;
pub static DT : f32 = 0.01;

fn test_force(node_index: usize, force_index: usize, data: &Vec<Node>) -> Vec<(usize,[f32;6])> {
    vec![(data[node_index].id,[0.01, 0., 0., 0., 0., 0.,])]
}

fn no_force(node_index: usize, force_index: usize, data: &Vec<Node>) -> Vec<(usize,[f32;6])> {
    vec![(0,[0., 0., 0., 0., 0., 0.,])]
}

pub struct ForceLibrary {
    //map: HashMap<String, fn(node_index: usize, force_index: usize, data: &Vec<Node>) -> Vec<(usize,[f32;6])>>,
    spring: fn(node_index: usize, force_index: usize, data: &Vec<Node>) -> Vec<(usize,[f32;6])>,
    //valence_angle: fn(node:Node, data: Vec<Node>, params: [f32;3]) -> Vec<[f32;3]>,
    test: fn(node_index: usize, force_index: usize, data: &Vec<Node>) -> Vec<(usize,[f32;6])>,
}

impl ForceLibrary {
    pub fn new() -> Self {
        Self{
            spring: harmonic::spring::spring_force,
            test: test_force,
        }
    }
}

pub fn resolve_forces(data: &mut Vec<Node>){
    let force_lib = ForceLibrary::new();
    for i in 0..data.len() {
        for j in 0..(data[i].forces).len() {

            let name : &str = &data[i].forces[j].name;
            let actions : Vec<(usize,[f32;6])> = match name {
                "test"=> test_force(i, j,&data),
                "spring"=> harmonic::spring::spring_force(i, j,&data),
                _ => no_force(i, j,&data),
            };
            for k in 0..actions.len() {
                let action = actions[k];
                let index = action.0;
                let force = action.1;
                let node = &mut (data[index]);
                node.net_force[0] += force[0];
                node.net_force[1] += force[1];
                node.net_force[2] += force[2];
                node.net_force[3] += force[3];
                node.net_force[4] += force[4];
                node.net_force[5] += force[5];
            }
        }
    }
}

pub fn update_state(data : &mut Vec<Node>) {
    for i in 0..data.len() {
        data[i].position[0] = data[i].position[3];
        data[i].position[1] = data[i].position[4];
        data[i].position[2] = data[i].position[5];

        data[i].velocity[0] = data[i].velocity[3];
        data[i].velocity[1] = data[i].velocity[4];
        data[i].velocity[2] = data[i].velocity[5];
    }
}

pub fn velocity_verlet(data: &mut Vec<Node>){

    for i in 0..data.len() {
        let node = &mut data[i];

        let position = [node.position[0],node.position[1],node.position[2],];
        let velocity = [node.velocity[0],node.velocity[1],node.velocity[2],];
        let net_force = [node.net_force[0],node.net_force[1],node.net_force[2],];
        let net_future_force = [node.net_force[3],node.net_force[4],node.net_force[5],];

        let v_half = vec::add(position,
            vec::scale(0.5 * (1. / node.mass) * 0.01,
            net_force));

        let rf = vec::add(position, vec::scale(DT, v_half));
        let vf = vec::add(v_half, vec::scale(DT * 0.5 * (1. / node.mass), net_future_force));

        node.position[3] = rf[0];
        node.position[4] = rf[1];
        node.position[5] = rf[2];
        node.velocity[3] = vf[0];
        node.velocity[4] = vf[1];
        node.velocity[5] = vf[2];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_forces() {
        let mut data : Vec<Node> = vec![];
        for i in 0..100 {
            let mut node = Node::new();
            node.id = i as usize;
            let force = Force {
                name: String::from("test"),
                params: [0., 0., 0.],
                indices: [0,0],
            };
            node.forces.push(force);
            data.push(node);
        }
        resolve_forces(&mut data);
        for node in data.iter() {
            assert_eq!(node.net_force[0], 0.01);
        }
    }

    #[test]
    fn test_velocity_verlet() {
        let mut data : Vec<Node> = vec![];
        for i in 0..100 {
            let mut node = Node::new();
            node.id = i as usize;
            let force = Force {
                name: String::from("test"),
                params: [0., 0., 0.],
                indices: [0,0],
            };
            node.forces.push(force);
            data.push(node);
        }
        resolve_forces(&mut data);
        velocity_verlet(&mut data);
        for node in data.iter() {
            // assert is equal to ut + 1/2 at^2
            assert_eq!(node.position[3], 0.5 * 0.01*DT*DT);
            // assert is equal to u + 1/2 at
            assert_eq!(node.velocity[3], 0.5 * 0.01 * DT);
        }
    }

    #[test]
    fn test_update_state() {
        let mut data : Vec<Node> = vec![];
        for i in 0..100 {
            let mut node = Node::new();
            node.id = i as usize;
            let force = Force {
                name: String::from("test"),
                params: [0., 0., 0.],
                indices: [0,0],
            };
            node.forces.push(force);
            data.push(node);
        }
        resolve_forces(&mut data);
        velocity_verlet(&mut data);
        update_state(&mut data);
        for node in data.iter() {
            // assert is equal to ut + 1/2 at^2
            assert_eq!(node.position[0], 0.5 * 0.01*DT*DT);
            // assert is equal to u + 1/2 at
            assert_eq!(node.velocity[0], 0.5 * 0.01 * DT);
        }
    }
}
