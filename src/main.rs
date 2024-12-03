
#[cfg(feature = "custom_cursor")]
use bevy::winit::cursor::CustomCursor;
use bevy::{
  core::FrameCount, diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, input::{gamepad::{
    GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadButtonStateChangedEvent,
    GamepadConnectionEvent, GamepadEvent,
  }, keyboard::KeyboardInput}, prelude::*, ui::update, window::{CompositeAlphaMode, CursorGrabMode, PresentMode, SystemCursorIcon, WindowLevel, WindowTheme}, winit::cursor::CursorIcon
};

mod print_input;
use print_input::*;
mod window_utils;
use window_utils::*;
use rand::{Rng};

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
    .add_systems(Startup, setup_particles)
    .add_systems(Update, update_particles)
    .run();
}


#[derive(Component, Default)]
struct ParticleDynamic {
  velocity: Vec2,
  acceleration: Vec2,
  jerk: Vec2,
  snap: Vec2,
  crackle: Vec2,
  pop: Vec2
}

// https://bevyengine.org/examples/2d-rendering/2d-shapes/
fn setup_particles(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  commands.spawn(Camera2d);
  let mut rng = rand::thread_rng();

  for _ in 0..1000 {
    commands.spawn(
      ( // Entity with a Mesh2d, MeshMaterial2d, Transform, and a ParticleDynamic
        Mesh2d(
          meshes.add(Circle::new(1.))
        ),
        MeshMaterial2d(
          materials.add(
            Color::hsv(
              rng.gen_range(0.0..1.0),
              1.0,
              1.0
            )
          )
        ),
        Transform::from_xyz(
          rng.gen_range(-256.0..256.0),
          rng.gen_range(-256.0..256.0),
          rng.gen_range(-256.0..256.0),
        ),
        ParticleDynamic {
          velocity: Vec2 {
            x: rng.gen_range(-16.0..16.0),
            y: rng.gen_range(-16.0..16.0)
          },
          ..Default::default()
        } // holy shit lol, this works?
      )
    );
  }

}

// Every entity is modified using a function like this, with a Query,
// where `Query<(&components, &go, &here)>` selects what entities are modified.
fn update_particles(
  time: Res<Time>,
  mut query: Query<(&ParticleDynamic, &mut Transform)>
) {
  // Practically, you will start every Update system with a Query like this
  for (dynamics, mut transform) in &mut query {
    transform.translation.x += dynamics.velocity.x * time.delta_secs();
    transform.translation.y += dynamics.velocity.y * time.delta_secs();
  }
}

// todo: Add a function with a Query?

/*
TODO:
- Genuary "particles"
- WASM export, selfhost
- https://bevyengine.org/examples/3d-rendering/parallax-mapping/ holy shit-- this can end up looking very cool
*/
