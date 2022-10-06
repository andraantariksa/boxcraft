use nalgebra::Vector2;
use std::collections::HashSet;

use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::window::Window;

pub struct InputManager {
    mouse_movement: Vector2<f32>,
    keyboard_pressed: HashSet<VirtualKeyCode>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            mouse_movement: Vector2::new(0.0, 0.0),
            keyboard_pressed: HashSet::new(),
        }
    }

    pub fn is_key_pressed(&self, key: &VirtualKeyCode) -> bool {
        self.keyboard_pressed.contains(key)
    }

    pub fn get_mouse_movement(&self) -> &Vector2<f32> {
        &self.mouse_movement
    }

    pub fn record_event(
        &mut self,
        window: &Window,
        window_event: &WindowEvent,
        is_cursor_locked: bool,
    ) {
        match window_event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode,
                        state,
                        ..
                    },
                ..
            } => {
                if let Some(v_key_code) = virtual_keycode {
                    match state {
                        ElementState::Pressed => {
                            self.keyboard_pressed.insert(*v_key_code);
                        }
                        ElementState::Released => {
                            self.keyboard_pressed.remove(v_key_code);
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if is_cursor_locked {
                    let window_size = window.inner_size();
                    let center = Vector2::<f32>::new(
                        window_size.width as f32 / 2.0,
                        window_size.height as f32 / 2.0,
                    );
                    let new_pos = Vector2::<f32>::new(position.x as f32, position.y as f32);

                    self.mouse_movement += center - new_pos;

                    #[warn(unused_must_use)]
                    window.set_cursor_position(PhysicalPosition {
                        x: center.x,
                        y: center.y,
                    });
                }
            }
            WindowEvent::MouseInput { .. } => {}
            _ => {}
        };
    }

    pub fn clear(&mut self) {
        self.mouse_movement = Vector2::zeros();
        self.keyboard_pressed.clear();
    }
}
