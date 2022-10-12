use crate::render::{ColoredMesh, Mesh};
use crate::render::{Colored_Mesh_Vertex, Mesh_Vertex};

pub struct MeshList {
    pub triangle:ColoredMesh,
    pub ground_plane:Mesh,
}

impl MeshList {
    pub fn new() -> MeshList {
        let vertices: Vec<Colored_Mesh_Vertex> = vec![
            Colored_Mesh_Vertex { pos: (-0.5, -0.5, -4.0).into(), clr: (1.0, 0.0, 0.0).into() },
            Colored_Mesh_Vertex { pos: (0.5, -0.5, -4.0).into(),  clr: (0.0, 1.0, 0.0).into() },
            Colored_Mesh_Vertex { pos: (0.0, 0.5, -4.0).into(),   clr: (0.0, 0.0, 1.0).into() },

            Colored_Mesh_Vertex { pos: (0.5, -0.5, -4.0).into(), clr: (1.0, 0.0, 0.0).into() },
            Colored_Mesh_Vertex { pos: (1.5, -0.5, -4.0).into(), clr: (0.0, 1.0, 0.0).into() },
            Colored_Mesh_Vertex { pos: (1.0, 0.5, -4.0).into(),  clr: (0.0, 0.0, 1.0).into() },

            Colored_Mesh_Vertex { pos: (0.0, 0.5, -4.0).into(), clr: (1.0, 0.0, 0.0).into() },
            Colored_Mesh_Vertex { pos: (1.0, 0.5, -4.0).into(), clr: (0.0, 1.0, 0.0).into() },
            Colored_Mesh_Vertex { pos: (0.5, 1.5, -4.0).into(),  clr: (0.0, 0.0, 1.0).into() },
        ];
        let triangle = ColoredMesh::from_vec(&vertices).unwrap();

        let vertices: Vec<Mesh_Vertex> = vec![
            Mesh_Vertex { pos: (-20.0, -0.5, -20.0).into(), tex: (0.0, 0.0).into(), c: 1.0 },
            Mesh_Vertex { pos: ( 20.0, -0.5,  20.0).into(), tex: (1.0, 1.0).into(), c: 1.0 },
            Mesh_Vertex { pos: ( 20.0, -0.5, -20.0).into(), tex: (1.0, 0.0).into(), c: 1.0 },
            Mesh_Vertex { pos: ( 20.0, -0.5,  20.0).into(), tex: (1.0, 1.0).into(), c: 1.0 },
            Mesh_Vertex { pos: (-20.0, -0.5, -20.0).into(), tex: (0.0, 0.0).into(), c: 1.0 },
            Mesh_Vertex { pos: (-20.0, -0.5,  20.0).into(), tex: (0.0, 1.0).into(), c: 1.0 },
        ];
        let ground_plane = Mesh::from_vec(&vertices).unwrap();

        MeshList {
            triangle,
            ground_plane,
        }
    }
}