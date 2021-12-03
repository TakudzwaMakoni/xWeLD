use super::harmonic as harmonic;
use super::verlet as verlet;
#[derive(Debug)]
pub struct Force {
    pub name: String,
    pub params: [f32;3],
    pub indices: [usize;2],
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub position: [f32;6],
    pub velocity: [f32;6],
    pub net_force: [f32;6],
    pub forces: Vec<Force>,
    pub visible: bool,
    pub colour: [f32;3],
    pub mass: f32,
    pub radius: f32,
    pub id : usize,
}

impl Node {
    pub fn new() -> Self {

        Self {
            name: String::from("basic node"),
            position: [0.;6],
            velocity: [0.;6],
            net_force: [0.;6],
            forces: vec![Force{name:"test".to_string(), params:[0.,0.,0.], indices: [0,0]}],
            visible: true,
            colour: [1.,0.,0.],
            mass: 1.,
            radius:1.,
            id: 0,
        }
    }
}

pub fn generate_interatomic_forces(force : &Force, data : &mut Vec<Node>, predicate : &str ) {

    let predicate_fn = match predicate {
        "spring" => harmonic::spring::basic_spring_predicate,
        _ => |d1 : &Node, d2: &Node, equilibrium : f32| -> bool { false },
    };

    for i in 0..data.len() {
        for j in 0..data.len(){
            if predicate_fn(&data[i], &data[j], 0.1 /*TODO*/) && i!=j {
                let id = data[j].id;
                data[i].forces.push(
                    Force {
                        name: String::from(&force.name),
                        params: force.params,
                        indices: [id, 0],
                    }
                );
            }
        }
    }
}



pub fn face_centred_cubic(cells_x: u8, cells_y: u8, cells_z: u8 , unit_len: f32) -> Vec<Node> {
    let mut data = vec![];
    let mut counter: usize = 0;
    for k in 0..cells_z {
        for j in 0..cells_y {
            for i in 0..cells_x {
                let mut node = Node::new();
                node.position =
                [
                (unit_len * i as f32) + (0.5 * unit_len),
                (unit_len * j as f32),
                (unit_len * k as f32) + (0.5 * unit_len),
                0., 0., 0.,
                ];
                node.id = counter;
                data.push(node);
                counter+=1;
            }
        }
    }

    data
}
