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

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  if true {
    // cube
    commands.spawn((
      Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
      MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
      Transform::from_xyz(8.0, 10.0, 8.0),
    ));
  }
  // light
  commands.spawn((
    PointLight {
      shadows_enabled: true,
      ..default()
    },
    Transform::from_xyz(4.0, 14.0, 4.0),
  ));
  // camera
  commands.spawn((
    camera::camera_components(),
    Transform::from_xyz(-2.5, 12.5, 9.0).looking_at(Vec3::new(8.0, 10.0, 8.0), Vec3::Y),
  ));
}
