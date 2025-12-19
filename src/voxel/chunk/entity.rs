use crate::voxel::chunk::{material::ChunkMaterial, mesh::ChunkMeshData};
use bevy::prelude::*;

impl ChunkMeshData {
  pub fn create_entity(
    self,
    materials: &mut Assets<ChunkMaterial>,
    meshes: &mut Assets<Mesh>,
  ) -> (Mesh3d, MeshMaterial3d<ChunkMaterial>, Transform) {
    let material = materials.add(ChunkMaterial {});

    (
      Mesh3d(meshes.add(self.mesh)),
      MeshMaterial3d(material),
      Transform::from_translation(self.chunk_pos.as_vec3() * super::CHUNK_SIZE as f32),
    )
  }
}
