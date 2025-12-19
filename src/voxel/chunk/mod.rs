use bevy::prelude::*;
pub use material::ChunkMaterialPlugin;

use crate::voxel::chunk::{generation::ChunkBlockData, material::ChunkMaterial};

mod entity;
mod generation;
mod material;
mod mesh;

const CHUNK_SIZE: usize = 16;
const CHUNK_SIZE_POW: usize = 5; // log2(16) = 4, plus 1 for first bit

#[derive(Component)]
pub struct Marker;

pub fn test(
  query: Query<Entity, With<Marker>>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ChunkMaterial>>,
) {
  if let Ok(entity) = query.single() {
    commands.entity(entity).despawn();
  }

  for x in -10..10 {
    for z in -10..10 {
      for y in -2..=1 {
        let chunk_pos = IVec3::new(x, y, z);
        let mesh_entity = ChunkBlockData::create(0, chunk_pos)
          .create_mesh()
          .create_entity(&mut materials, &mut meshes);

        commands.spawn((Marker, mesh_entity));
      }
    }
  }
}
