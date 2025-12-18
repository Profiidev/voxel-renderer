use crate::voxel::chunk::{
  material::{ChunkMaterial, ChunkMaterialHandle},
  mesh::ChunkMeshData,
};
use bevy::prelude::*;

impl ChunkMeshData {
  pub fn create_entity(
    self,
    materials: &mut Assets<ChunkMaterial>,
    meshes: &mut Assets<Mesh>,
    commands: &mut Commands,
  ) -> (Mesh3d, MeshMaterial3d<ChunkMaterial>, Transform) {
    let material = materials.add(ChunkMaterial {});
    commands.insert_resource(ChunkMaterialHandle(material.clone()));

    (
      Mesh3d(meshes.add(self.mesh)),
      MeshMaterial3d(material),
      Transform::from_translation(self.chunk_pos.as_vec3() * super::CHUNK_SIZE as f32),
    )
  }
}
