use bevy::prelude::*;
pub use material::ChunkMaterialPlugin;

use crate::voxel::chunk::{
  generation::ChunkBlockData,
  material::{ChunkMaterial, ChunkMaterialHandle},
};

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

  let mesh = ChunkBlockData::create(42, IVec3::ZERO).mesh();
  let material = materials.add(ChunkMaterial {});
  commands.insert_resource(ChunkMaterialHandle(material.clone()));

  commands.spawn((
    Transform::from_xyz(0.0, 0.0, 0.0),
    Mesh3d(meshes.add(mesh)),
    MeshMaterial3d(material),
    Marker,
  ));
}
