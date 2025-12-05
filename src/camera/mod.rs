use bevy::prelude::*;

use crate::camera::controller::CameraController;

mod controller;
mod system;

pub fn camera_components() -> impl Bundle {
  (CameraController::default(), Camera3d::default())
}

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, system::run_camera_controller);
  }
}
