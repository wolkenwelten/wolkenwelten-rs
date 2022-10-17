use crate::Mesh;

#[derive(Debug, Default)]
pub struct MeshList {
    pub pear: Mesh,
}

impl MeshList {
    pub fn new() -> Self {
        let pear = Mesh::from_obj_string(include_str!("../assets/meshes/pear.obj")).unwrap();

        Self { pear }
    }
}
