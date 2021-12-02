
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
        }
    }
}



pub fn face_centred_cubic(cells_x: u8, cells_y: u8, cells_z: u8 , unit_len: f32) -> Vec<Node> {
    let mut data = vec![];
    for k in 0..cells_x {
        for j in 0..cells_x {
            for i in 0..cells_x {
                let mut node = Node::new();
                node.position =
                [
                (unit_len * i as f32) + (0.5 * unit_len),
                (unit_len * j as f32),
                (unit_len * k as f32) + (0.5 * unit_len),
                0., 0., 0.,
                ];
                data.push(node);
            }
        }
    }

    data
}
