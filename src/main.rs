use bevy::{
  color::palettes::css::WHITE,
  pbr::wireframe::{WireframeConfig, WireframePlugin},
  prelude::*,
};

use crate::{camera::CameraControllerPlugin, voxel::VoxelPlugin};

mod camera;
mod voxel;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(AssetPlugin {
      watch_for_changes_override: Some(true),
      ..Default::default()
    }))
    .add_plugins(CameraControllerPlugin)
    .add_plugins(WireframePlugin::default())
    .add_plugins(VoxelPlugin)
    .add_systems(Startup, setup)
    .insert_resource(WireframeConfig {
      global: true,
      default_color: WHITE.into(),
    })
    .run();
}

fn setup(mut commands: Commands) {
  // camera
  commands.spawn((
    camera::camera_components(),
    Transform::from_xyz(-2.5, 12.5, 9.0).looking_at(Vec3::new(8.0, 10.0, 8.0), Vec3::Y),
  ));
}
