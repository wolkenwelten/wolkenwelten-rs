use crate::render::Mesh;
use crate::render::Vertex;

pub struct MeshList {
    pub triangle:Mesh,
}

impl MeshList {
    pub fn new() -> MeshList {
        let vertices: Vec<Vertex> = vec![
            Vertex { pos: (-0.5, -0.5, -1.0).into(), clr: (1.0, 1.0, 0.0).into() },
            Vertex { pos: (0.5, -0.5, -1.0).into(),  clr: (0.0, 1.0, 0.0).into() },
            Vertex { pos: (0.0, 0.5, -1.0).into(),   clr: (0.0, 1.0, 1.0).into() },

            Vertex { pos: (-0.5, -0.5, -2.0).into(), clr: (1.0, 0.0, 1.0).into() },
            Vertex { pos: (0.5, -0.5, -2.0).into(),  clr: (0.0, 1.0, 0.0).into() },
            Vertex { pos: (0.0, 0.5, -2.0).into(),   clr: (1.0, 0.0, 1.0).into() },

            Vertex { pos: (-0.5, -0.5, -3.0).into(), clr: (1.0, 0.0, 0.0).into() },
            Vertex { pos: (0.5, -0.5, -3.0).into(),  clr: (1.0, 1.0, 0.0).into() },
            Vertex { pos: (0.0, 0.5, -3.0).into(),   clr: (0.0, 1.0, 1.0).into() },

            Vertex { pos: (-0.5, -0.5, -4.0).into(), clr: (1.0, 0.0, 0.0).into() },
            Vertex { pos: (0.5, -0.5, -4.0).into(),  clr: (0.0, 1.0, 0.0).into() },
            Vertex { pos: (0.0, 0.5, -4.0).into(),   clr: (0.0, 0.0, 1.0).into() },


            Vertex { pos: (0.5, -0.5, -1.0).into(), clr: (1.0, 1.0, 0.0).into() },
            Vertex { pos: (1.5, -0.5, -1.0).into(), clr: (0.0, 1.0, 0.0).into() },
            Vertex { pos: (1.0, 0.5, -1.0).into(),  clr: (0.0, 1.0, 1.0).into() },

            Vertex { pos: (0.5, -0.5, -2.0).into(), clr: (1.0, 0.0, 1.0).into() },
            Vertex { pos: (1.5, -0.5, -2.0).into(), clr: (0.0, 1.0, 0.0).into() },
            Vertex { pos: (1.0, 0.5, -2.0).into(),  clr: (1.0, 0.0, 1.0).into() },

            Vertex { pos: (0.5, -0.5, -3.0).into(), clr: (1.0, 0.0, 0.0).into() },
            Vertex { pos: (1.5, -0.5, -3.0).into(), clr: (1.0, 1.0, 0.0).into() },
            Vertex { pos: (1.0, 0.5, -3.0).into(),  clr: (0.0, 1.0, 1.0).into() },

            Vertex { pos: (0.5, -0.5, -4.0).into(), clr: (1.0, 0.0, 0.0).into() },
            Vertex { pos: (1.5, -0.5, -4.0).into(), clr: (0.0, 1.0, 0.0).into() },
            Vertex { pos: (1.0, 0.5, -4.0).into(),  clr: (0.0, 0.0, 1.0).into() },

            Vertex { pos: (0.0, 0.5, -4.0).into(), clr: (1.0, 0.0, 0.0).into() },
            Vertex { pos: (1.0, 0.5, -4.0).into(), clr: (0.0, 1.0, 0.0).into() },
            Vertex { pos: (0.5, 1.5, -4.0).into(),  clr: (0.0, 0.0, 1.0).into() },

            Vertex { pos: (-20.0, -0.5, -20.0).into(), clr: (1.0, 1.0, 0.0).into() },
            Vertex { pos: ( 20.0, -0.5,  20.0).into(), clr: (0.0, 0.0, 0.0).into() },
            Vertex { pos: ( 20.0, -0.5, -20.0).into(), clr: (0.0, 1.0, 0.0).into() },
            Vertex { pos: ( 20.0, -0.5,  20.0).into(), clr: (0.0, 0.0, 0.0).into() },
            Vertex { pos: (-20.0, -0.5, -20.0).into(), clr: (1.0, 1.0, 0.0).into() },
            Vertex { pos: (-20.0, -0.5,  20.0).into(), clr: (1.0, 1.0, 0.0).into() },


        ];
        let triangle = Mesh::from_vec(&vertices).unwrap();

        MeshList {
            triangle,
        }
    }
}