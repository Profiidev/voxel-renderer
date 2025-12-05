use bevy::prelude::*;

/// Camera controller [`Component`].
#[derive(Component)]
pub struct CameraController {
  /// Enables this [`CameraController`] when `true`.
  pub enabled: bool,
  /// Indicates if this controller has been initialized by the [`CameraControllerPlugin`].
  pub initialized: bool,
  /// Multiplier for pitch and yaw rotation speed.
  pub sensitivity: f32,
  /// [`KeyCode`] for forward translation.
  pub key_forward: KeyCode,
  /// [`KeyCode`] for backward translation.
  pub key_back: KeyCode,
  /// [`KeyCode`] for left translation.
  pub key_left: KeyCode,
  /// [`KeyCode`] for right translation.
  pub key_right: KeyCode,
  /// [`KeyCode`] for up translation.
  pub key_up: KeyCode,
  /// [`KeyCode`] for down translation.
  pub key_down: KeyCode,
  /// [`KeyCode`] to use [`run_speed`](CameraController::run_speed) instead of
  /// [`walk_speed`](CameraController::walk_speed) for translation.
  pub key_run: KeyCode,
  /// [`MouseButton`] for grabbing the mouse focus.
  pub mouse_key_cursor_grab: MouseButton,
  /// [`KeyCode`] for grabbing the keyboard focus.
  pub keyboard_key_toggle_cursor_grab: KeyCode,
  /// Multiplier for unmodified translation speed.
  pub walk_speed: f32,
  /// Multiplier for running translation speed.
  pub run_speed: f32,
  /// Multiplier for how the mouse scroll wheel modifies [`walk_speed`](CameraController::walk_speed)
  /// and [`run_speed`](CameraController::run_speed).
  pub scroll_factor: f32,
  /// Friction factor used to exponentially decay [`velocity`](CameraController::velocity) over time.
  pub friction: f32,
  /// This [`CameraController`]'s pitch rotation.
  pub pitch: f32,
  /// This [`CameraController`]'s yaw rotation.
  pub yaw: f32,
  /// This [`CameraController`]'s translation velocity.
  pub velocity: Vec3,
}

impl Default for CameraController {
  fn default() -> Self {
    Self {
      enabled: true,
      initialized: false,
      sensitivity: 1.0,
      key_forward: KeyCode::KeyW,
      key_back: KeyCode::KeyS,
      key_left: KeyCode::KeyA,
      key_right: KeyCode::KeyD,
      key_up: KeyCode::KeyE,
      key_down: KeyCode::KeyQ,
      key_run: KeyCode::ShiftLeft,
      mouse_key_cursor_grab: MouseButton::Left,
      keyboard_key_toggle_cursor_grab: KeyCode::KeyM,
      walk_speed: 5.0,
      run_speed: 15.0,
      scroll_factor: 0.1,
      friction: 0.5,
      pitch: 0.0,
      yaw: 0.0,
      velocity: Vec3::ZERO,
    }
  }
}
