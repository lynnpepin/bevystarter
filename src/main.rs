
#[cfg(feature = "custom_cursor")]
use bevy::winit::cursor::CustomCursor;
use bevy::{
  input::gamepad::{
    GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadButtonStateChangedEvent,
    GamepadConnectionEvent, GamepadEvent,
  },
  input::keyboard::KeyboardInput,
  core::FrameCount,
  diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
  window::{CursorGrabMode, PresentMode, SystemCursorIcon, WindowLevel, WindowTheme},
  winit::cursor::CursorIcon,
  prelude::*,
};

mod grist;
use grist::*;

fn main() {
  App::new()
    .add_plugins(
      (DefaultPlugins
        .set(ImagePlugin::default_nearest())
        // see: https://github.com/bevyengine/bevy/blob/main/examples/window/window_settings.rs
        .set(WindowPlugin {
          primary_window: Some(Window {
            title: "Bevy Starter Window".into(),
            name: Some("bevy_starter.app".into()),
            resolution: (640., 480.).into(),
            present_mode: PresentMode::AutoVsync,
            fit_canvas_to_parent: true,
            prevent_default_event_handling: false,
            window_theme: Some(WindowTheme::Light),
            enabled_buttons: bevy::window::EnabledButtons {
              maximize: false,
              minimize: true,
              close: true
            },
            visible: true,
            ..default()
          }),
          ..default()
        }),
      )
    )
    .add_systems(
      Update,
      (
        gamepad_events,
        gamepad_ordered_events,
        print_keyboard_event_system,
      ),
    )
    .run();
}


