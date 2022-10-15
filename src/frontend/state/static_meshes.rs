use crate::frontend::Mesh;
use crate::frontend::mesh::{BlockMesh, MeshVertex};

pub struct MeshList {
	pub ground_plane: Mesh,
	pub pear: Mesh,
	pub voxel_test: BlockMesh,
}

impl MeshList {
	pub fn new() -> Self {
		let vertices: Vec<MeshVertex> = vec![
			MeshVertex { pos: (-20.0, -0.5, -20.0).into(), tex: (0.0, 0.0).into(), c: 1.0 },
			MeshVertex { pos: (20.0, -0.5, 20.0).into(), tex: (1.0, 1.0).into(), c: 1.0 },
			MeshVertex { pos: (20.0, -0.5, -20.0).into(), tex: (1.0, 0.0).into(), c: 1.0 },
			MeshVertex { pos: (20.0, -0.5, 20.0).into(), tex: (1.0, 1.0).into(), c: 1.0 },
			MeshVertex { pos: (-20.0, -0.5, -20.0).into(), tex: (0.0, 0.0).into(), c: 1.0 },
			MeshVertex { pos: (-20.0, -0.5, 20.0).into(), tex: (0.0, 1.0).into(), c: 1.0 },
		];
		let ground_plane = Mesh::from_vec(&vertices).unwrap();

		let pear = Mesh::from_obj_string(include_str!("../assets/meshes/pear.obj")).unwrap();
		let voxel_test = BlockMesh::test_mesh();

		Self {
			ground_plane,
			pear,
			voxel_test,
		}
	}
}
