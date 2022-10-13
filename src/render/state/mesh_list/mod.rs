use crate::render::Mesh;
use crate::render::mesh::Mesh_Vertex;

pub struct MeshList {
	pub ground_plane: Mesh,
	pub pear: Mesh,
}

impl MeshList {
	pub fn new() -> Self {
		let vertices: Vec<Mesh_Vertex> = vec![
			Mesh_Vertex { pos: (-20.0, -0.5, -20.0).into(), tex: (0.0, 0.0).into(), c: 1.0 },
			Mesh_Vertex { pos: (20.0, -0.5, 20.0).into(), tex: (1.0, 1.0).into(), c: 1.0 },
			Mesh_Vertex { pos: (20.0, -0.5, -20.0).into(), tex: (1.0, 0.0).into(), c: 1.0 },
			Mesh_Vertex { pos: (20.0, -0.5, 20.0).into(), tex: (1.0, 1.0).into(), c: 1.0 },
			Mesh_Vertex { pos: (-20.0, -0.5, -20.0).into(), tex: (0.0, 0.0).into(), c: 1.0 },
			Mesh_Vertex { pos: (-20.0, -0.5, 20.0).into(), tex: (0.0, 1.0).into(), c: 1.0 },
		];
		let ground_plane = Mesh::from_vec(&vertices).unwrap();

		let pear = Mesh::from_obj_string(include_str!("./pear.obj")).unwrap();

		Self {
			ground_plane,
			pear,
		}
	}
}
