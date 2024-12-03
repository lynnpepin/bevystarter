
#[cfg(feature = "custom_cursor")]
use bevy::winit::cursor::CustomCursor;
use bevy::{
  core::FrameCount, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, input::{gamepad::{
    GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadButtonStateChangedEvent,
    GamepadConnectionEvent, GamepadEvent,
  }, keyboard::KeyboardInput}, prelude::*, window::{CompositeAlphaMode, CursorGrabMode, PresentMode, SystemCursorIcon, WindowLevel, WindowTheme}, winit::cursor::CursorIcon
};

mod print_input;
use print_input::*;
mod window_utils;
use window_utils::*;

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
            // see: https://github.com/bevyengine/bevy/blob/main/examples/window/transparent_window.rs
            // Setting `transparent` allows the `ClearColor`'s alpha value to take effect
            transparent: true,
            // Disabling window decorations to make it feel more like a widget than a window
            decorations: true,
            #[cfg(target_os = "macos")]
            composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
            #[cfg(target_os = "linux")]
            composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
            // ClearColor must have 0 alpha, otherwise some color will bleed through
            visible: false,
            ..default()
          }),
          ..default()
        }),
      )
    )
    .insert_resource(ClearColor(Color::NONE)) // for transparent window
    .add_systems(
      Update,
      (
        gamepad_events,
        gamepad_ordered_events,
        print_keyboard_event_system,
      ),
    )
    .add_systems(
      Update,
      (
        change_title,
        toggle_theme,
        toggle_cursor,
        toggle_vsync,
        toggle_window_controls,
        cycle_cursor_icon,
        switch_level,
        make_visible,
      )
    )
    .add_systems(Startup, setup)
    .run();
}


// https://bevyengine.org/examples/2d-rendering/2d-shapes/
fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  commands.spawn(Camera2d);

  commands.spawn(
    (
      Mesh2d(
        meshes.add(Circle::new(4.))
      ),
      MeshMaterial2d(
        materials.add(
          Color::hsv(0.0, 1.0, 1.0)
        )
      ),
      Transform::from_xyz(8.0, 8.0, 0.0)
    )
  );

}

/*
TODO:
- Genuary "particles"
- WASM export, selfhost
*/
