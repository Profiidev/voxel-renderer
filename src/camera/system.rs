use std::f32::consts::PI;

use bevy::{
  input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit},
  prelude::*,
  window::{CursorGrabMode, CursorOptions},
};

use crate::camera::controller::CameraController;

pub const RADIANS_PER_DOT: f32 = 1.0 / 180.0;

#[allow(clippy::too_many_arguments)]
pub fn run_camera_controller(
  time: Res<Time<Real>>,
  mut windows: Query<(&Window, &mut CursorOptions)>,
  accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
  accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
  mouse_button_input: Res<ButtonInput<MouseButton>>,
  key_input: Res<ButtonInput<KeyCode>>,
  mut toggle_cursor_grab: Local<bool>,
  mut mouse_cursor_grab: Local<bool>,
  mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
  let dt = time.delta_secs();

  let Ok((mut transform, mut controller)) = query.single_mut() else {
    info!("No camera with CameraController found, skipping CameraController system.");
    return;
  };

  if !controller.initialized {
    let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
    controller.yaw = yaw;
    controller.pitch = pitch;
    controller.initialized = true;
  }
  if !controller.enabled {
    return;
  }

  let mut scroll = 0.0;

  let amount = match accumulated_mouse_scroll.unit {
    MouseScrollUnit::Line => accumulated_mouse_scroll.delta.y,
    MouseScrollUnit::Pixel => accumulated_mouse_scroll.delta.y / 16.0,
  };
  scroll += amount;
  controller.walk_speed += scroll * controller.scroll_factor * controller.walk_speed;
  controller.run_speed = controller.walk_speed * 3.0;

  // Handle key input
  let mut axis_input = Vec3::ZERO;
  if key_input.pressed(controller.key_forward) {
    axis_input.z += 1.0;
  }
  if key_input.pressed(controller.key_back) {
    axis_input.z -= 1.0;
  }
  if key_input.pressed(controller.key_right) {
    axis_input.x += 1.0;
  }
  if key_input.pressed(controller.key_left) {
    axis_input.x -= 1.0;
  }
  if key_input.pressed(controller.key_up) {
    axis_input.y += 1.0;
  }
  if key_input.pressed(controller.key_down) {
    axis_input.y -= 1.0;
  }

  let mut cursor_grab_change = false;
  if key_input.just_pressed(controller.keyboard_key_toggle_cursor_grab) {
    *toggle_cursor_grab = !*toggle_cursor_grab;
    cursor_grab_change = true;
  }
  if mouse_button_input.just_pressed(controller.mouse_key_cursor_grab) {
    *mouse_cursor_grab = true;
    cursor_grab_change = true;
  }
  if mouse_button_input.just_released(controller.mouse_key_cursor_grab) {
    *mouse_cursor_grab = false;
    cursor_grab_change = true;
  }
  let cursor_grab = *mouse_cursor_grab || *toggle_cursor_grab;

  // Update velocity
  if axis_input != Vec3::ZERO {
    let max_speed = if key_input.pressed(controller.key_run) {
      controller.run_speed
    } else {
      controller.walk_speed
    };
    controller.velocity = axis_input.normalize() * max_speed;
  } else {
    let friction = controller.friction.clamp(0.0, 1.0);
    controller.velocity *= 1.0 - friction;
    if controller.velocity.length_squared() < 1e-6 {
      controller.velocity = Vec3::ZERO;
    }
  }

  // Apply movement update
  if controller.velocity != Vec3::ZERO {
    let forward = *transform.forward();
    let right = *transform.right();
    transform.translation += controller.velocity.x * dt * right
      + controller.velocity.y * dt * Vec3::Y
      + controller.velocity.z * dt * forward;
  }

  // Handle cursor grab
  if cursor_grab_change {
    if cursor_grab {
      for (window, mut cursor_options) in &mut windows {
        if !window.focused {
          continue;
        }

        cursor_options.grab_mode = CursorGrabMode::Locked;
        cursor_options.visible = false;
      }
    } else {
      for (_, mut cursor_options) in &mut windows {
        cursor_options.grab_mode = CursorGrabMode::None;
        cursor_options.visible = true;
      }
    }
  }

  // Handle mouse input
  if accumulated_mouse_motion.delta != Vec2::ZERO && cursor_grab {
    // Apply look update
    controller.pitch = (controller.pitch
      - accumulated_mouse_motion.delta.y * RADIANS_PER_DOT * controller.sensitivity)
      .clamp(-PI / 2., PI / 2.);
    controller.yaw -= accumulated_mouse_motion.delta.x * RADIANS_PER_DOT * controller.sensitivity;
    transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, controller.yaw, controller.pitch);
  }
}
