use bevy::prelude::*;

use crate::voxel::chunk::{ChunkMaterialPlugin, test};

mod chunk;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, test)
      .add_plugins(ChunkMaterialPlugin::default());
  }
}
