use bevy::prelude::*;

use crate::voxel::chunk::{ChunkMaterial, ChunkMaterialHandle, ChunkMaterialPlugin};

mod chunk;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, test)
      .add_plugins(ChunkMaterialPlugin::default());
  }
}

fn test(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ChunkMaterial>>,
) {
  let mesh = chunk::ChunkData::new().mesh();
  let material = materials.add(ChunkMaterial {});
  commands.insert_resource(ChunkMaterialHandle(material.clone()));

  commands.spawn((
    Transform::from_xyz(0.0, 0.0, 0.0),
    Mesh3d(meshes.add(mesh)),
    MeshMaterial3d(material),
  ));
}
