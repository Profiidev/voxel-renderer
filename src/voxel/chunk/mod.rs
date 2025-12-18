use bevy::prelude::*;
pub use material::ChunkMaterialPlugin;

use crate::voxel::chunk::{generation::ChunkBlockData, material::ChunkMaterial};

mod entity;
mod generation;
mod material;
mod mesh;

const CHUNK_SIZE: usize = 16;
const CHUNK_SIZE_POW: usize = 5; // log2(32) = 5, plus 1 for first bit

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

  let chunk_pos = IVec3::new(0, 0, 0);
  let mesh_entity = ChunkBlockData::create(42, chunk_pos)
    .create_mesh()
    .create_entity(&mut materials, &mut meshes, &mut commands);

  commands.spawn((Marker, mesh_entity));
}
