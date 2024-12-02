
use bevy::{
  input::gamepad::{
    GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadButtonInput, GamepadConnectionEvent,
    GamepadEvent,
  },
  input::keyboard::KeyboardInput,
  prelude::*,
};

mod grist;
use grist::*;

fn main() {
  App::new()
    .add_plugins(
      (
        DefaultPlugins
          .set(ImagePlugin::default_nearest())
      ),
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


