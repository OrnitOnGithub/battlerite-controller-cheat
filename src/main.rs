use enigo::{
  Direction::{Press, Release},
  Coordinate,
  Settings,
  Keyboard,
  Button,
  Mouse,
  Enigo,
  Key,
};
use gilrs::*;

const SCREEN_HEIGHT: i32 = 1080;
const SCREEN_WIDTH:  i32 = 1920;
const CONTROLLER_DEADZONE: f32 = 0.3;

fn main() {
  let mut enigo: Enigo = Enigo::new(& Settings::default()).unwrap();
  let mut gilrs: Gilrs = Gilrs::new().unwrap();
  let mut active_gamepad: Option<GamepadId> = None;

  loop {
    while let Some(Event { id, event: _, time: _ }) = gilrs.next_event() {
      active_gamepad = Some(id);
    }

    let mut mouse_position: Vector2 = Vector2::new();
    let mut movement_direction: Vector2 = Vector2::new();

    if let Some(gamepad) = active_gamepad.map(|id| gilrs.gamepad(id)) {
      match gamepad.axis_data(Axis::RightStickX)  {
        Some(axis_data) => {
          mouse_position.x = ((axis_data.value()) * (SCREEN_HEIGHT + 400) as f32) as i32 / 2 + (SCREEN_WIDTH/2);
        } _ => {}
      }
      match gamepad.axis_data(Axis::RightStickY)  {
        Some(axis_data) => {
          mouse_position.y = ((-axis_data.value()) * (SCREEN_HEIGHT) as f32) as i32 / 2 + (SCREEN_HEIGHT/2);
        } _ => {}
      }
      match gamepad.axis_data(Axis::LeftStickX)  {
        Some(axis_data) => {
          if axis_data.value() > CONTROLLER_DEADZONE {
            let _ = enigo.key(Key::Unicode('d'), Press);
          } else {
            let _ = enigo.key(Key::Unicode('d'), Release);
          }
          if axis_data.value() < -CONTROLLER_DEADZONE {
            let _ = enigo.key(Key::Unicode('a'), Press);
          } else {
            let _ = enigo.key(Key::Unicode('a'), Release);
          }
          movement_direction.x = ((axis_data.value()) * (SCREEN_HEIGHT + 400) as f32) as i32 / 2 + (SCREEN_WIDTH/2);
        } _ => {}
      }
      match gamepad.axis_data(Axis::LeftStickY)  {
        Some(axis_data) => {
          if axis_data.value() > CONTROLLER_DEADZONE {
            let _ = enigo.key(Key::Unicode('w'), Press);
          }
          else {
            let _ = enigo.key(Key::Unicode('w'), Release);
          }
          if axis_data.value() < -CONTROLLER_DEADZONE {
            let _ = enigo.key(Key::Unicode('s'), Press);
          }
          else {
            let _ = enigo.key(Key::Unicode('s'), Release);
          }
          movement_direction.y = ((-axis_data.value()) * (SCREEN_HEIGHT) as f32) as i32 / 2 + (SCREEN_HEIGHT/2);
        } _ => {}
      }
      match gamepad.button_data(gilrs::Button::RightTrigger2) {
        Some(button_data) => {
          if button_data.value() > 0.2 {
            let _ = enigo.button(Button::Left, Press);
          } else {
            let _ = enigo.button(Button::Left, Release);
          }
        } _ => {}
      }
      match gamepad.button_data(gilrs::Button::LeftTrigger2) {
        Some(button_data) => {
          if button_data.value() > 0.2 {
            let _ = enigo.button(Button::Right, Press);
          } else {
            let _ = enigo.button(Button::Right, Release);
          }
        } _ => {}
      }
      match gamepad.button_data(gilrs::Button::LeftTrigger) {
        Some(button_data) => {
          if button_data.value() > 0.2 {
            let _ = enigo.key(Key::Unicode('q'), Press);
          } else {
            let _ = enigo.key(Key::Unicode('q'), Release);
          }
        } _ => {}
      }
      match gamepad.button_data(gilrs::Button::RightTrigger) {
        Some(button_data) => {
          if button_data.value() > 0.2 {
            let _ = enigo.key(Key::Unicode('e'), Press);
          } else {
            let _ = enigo.key(Key::Unicode('e'), Release);
          }
        } _ => {}
      }
      match gamepad.button_data(gilrs::Button::DPadDown) {
        Some(button_data) => {
          if button_data.value() > 0.2 {
            let _ = enigo.key(Key::Unicode('r'), Press);
          } else {
            let _ = enigo.key(Key::Unicode('r'), Release);
          }
        } _ => {}
      }
      match gamepad.button_data(gilrs::Button::DPadUp) {
        Some(button_data) => {
          if button_data.value() > 0.2 {
            let _ = enigo.key(Key::Unicode('f'), Press);
          } else {
            let _ = enigo.key(Key::Unicode('f'), Release);
          }
        } _ => {}
      }
      match gamepad.button_data(ev::Button::South) {
        Some(button_data) => {
          if button_data.value() > 0.2 {
            mouse_position = movement_direction;
            let _ = enigo.key(Key::Space, Press);
          } else {
            let _ = enigo.key(Key::Space, Release);
          }
        } _ => {}
      }
      match gamepad.button_data(ev::Button::Start) {
        Some(button_data) => {
          if button_data.value() > 0.2 {
            let _ = enigo.key(Key::Escape, Press);
          } else {
            let _ = enigo.key(Key::Escape, Release);
          }
        } _ => {}
      }
    }
    let _ = enigo.move_mouse(mouse_position.x, mouse_position.y, Coordinate::Abs);
  }
}

struct Vector2 {
  x: i32,
  y: i32,
}
impl Vector2 {
  fn new() -> Vector2 {
    return Vector2 { x: 0, y: 0 };
  }
}