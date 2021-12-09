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
            forces: vec![],
            visible: true,
            colour: [1.,0.,0.],
            mass: 1.,
            radius:1.,
            id: 0,
        }
    }
}

#[allow(dead_code)]
pub fn primitive_cubic(cells_x: u8, cells_y: u8, cells_z: u8 , unit_len: f32) -> Vec<Node> {
    let mut data = vec![];
    let mut counter: usize = 0;
    for k in 0..cells_z {
        for j in 0..cells_y {
            for i in 0..cells_x {
                let mut node1 = Node::new();
                node1.position =
                [
                (unit_len * i as f32),
                (unit_len * j as f32),
                (unit_len * k as f32),
                0., 0., 0.,
                ];
                node1.id = counter;
                data.push(node1);
                counter+=1;
            }
        }
    }
    data
}

#[allow(dead_code)]
pub fn face_centred_cubic(cells_x: u8, cells_y: u8, cells_z: u8 , unit_len: f32) -> Vec<Node> {
    let mut data = vec![];
    let mut counter: usize = 0;
    for k in 0..cells_z {
        for j in 0..cells_y {
            for i in 0..cells_x {
                let mut node1 = Node::new();
                node1.position =
                [
                (unit_len * i as f32) + (0.5 * unit_len),
                (unit_len * j as f32),
                (unit_len * k as f32) + (0.5 * unit_len),
                0., 0., 0.,
                ];
                node1.id = counter;
                data.push(node1);
                counter+=1;

                let mut node2 = Node::new();
                node2.position =
                [
                (unit_len * i as f32),
                (unit_len * j as f32) + (0.5 * unit_len),
                (unit_len * k as f32) + (0.5 * unit_len),
                0., 0., 0.,
                ];
                node2.id = counter;
                data.push(node2);
                counter+=1;

                let mut node3 = Node::new();
                node3.position =
                [
                (unit_len * i as f32) + (0.5 * unit_len),
                (unit_len * j as f32) + (0.5 * unit_len),
                (unit_len * k as f32),
                0., 0., 0.,
                ];
                node3.id = counter;
                data.push(node3);
                counter+=1;
            }
        }
    }
    data
}
