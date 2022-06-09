use nalgebra::{Vector2, Vector3};
use std::collections::HashSet;
use std::rc::Rc;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::window::Window;

pub struct InputManager {
    delta_mouse: Vector2<f32>,
    keyboard_pressed: HashSet<VirtualKeyCode>,
    window: Rc<Window>,
}

impl InputManager {
    pub fn new(window: Rc<Window>) -> Self {
        Self {
            delta_mouse: Vector2::new(0.0, 0.0),
            keyboard_pressed: HashSet::new(),
            window,
        }
    }

    pub fn record(&mut self, window_event: &WindowEvent) {
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
            WindowEvent::CursorMoved { position, .. } => {}
            WindowEvent::MouseInput { .. } => {}
            _ => {}
        };
    }

    pub fn clear(&mut self) {
        self.delta_mouse = Vector2::new(0.0, 0.0);
        self.keyboard_pressed.clear();
    }
}
