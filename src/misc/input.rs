use nalgebra::Vector2;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use bevy_ecs::prelude::*;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::window::Window;

#[derive(PartialEq)]
enum DoublePressState {
    FirstPress,
    FirstRelease,
}

struct DoublePress {
    timestamp: f32,
    kind: DoublePressState,
}

#[derive(Resource)]
pub struct InputManager {
    mouse_movement: Vector2<f32>,
    keyboard_pressed: HashSet<VirtualKeyCode>,
    double_press_states: HashMap<VirtualKeyCode, DoublePress>,
    double_pressed: HashSet<VirtualKeyCode>,
}

const DOUBLE_PRESS_MAX_INTERVAL: f32 = 300.0;

impl InputManager {
    pub fn new() -> Self {
        Self {
            mouse_movement: Vector2::new(0.0, 0.0),
            keyboard_pressed: HashSet::new(),
            double_press_states: HashMap::new(),
            double_pressed: HashSet::new(),
        }
    }

    pub fn is_key_pressed(&self, key: &VirtualKeyCode) -> bool {
        self.keyboard_pressed.contains(key)
    }

    pub fn is_double_pressed(&self, key: &VirtualKeyCode) -> bool {
        self.double_pressed.contains(key)
    }

    pub fn get_mouse_movement(&self) -> &Vector2<f32> {
        &self.mouse_movement
    }

    fn resolve_double_press(
        &mut self,
        element_state: &ElementState,
        v_key_code: &VirtualKeyCode,
        timestamp: f32,
    ) {
        match element_state {
            ElementState::Pressed => {
                match self.double_press_states.entry(*v_key_code) {
                    Entry::Occupied(entry) => {
                        if entry.get().kind == DoublePressState::FirstRelease {
                            let entry = entry.remove();

                            if timestamp - entry.timestamp < DOUBLE_PRESS_MAX_INTERVAL {
                                self.double_pressed.insert(*v_key_code);
                            }
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(DoublePress {
                            timestamp,
                            kind: DoublePressState::FirstPress,
                        });
                    }
                };
            }
            ElementState::Released => {
                let mut queue_to_delete = false;

                if let Some(entry) = self.double_press_states.get_mut(v_key_code) {
                    if timestamp - entry.timestamp < DOUBLE_PRESS_MAX_INTERVAL
                        && entry.kind == DoublePressState::FirstPress
                    {
                        entry.kind = DoublePressState::FirstRelease;
                    } else {
                        queue_to_delete = true;
                    }
                }

                if queue_to_delete {
                    self.double_press_states.remove(v_key_code);
                }
            }
        }
    }

    pub fn record_event(
        &mut self,
        window: &Window,
        window_event: &WindowEvent,
        is_cursor_locked: bool,
        timestamp: f32,
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
                    self.resolve_double_press(state, v_key_code, timestamp);
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
        self.double_pressed.clear();
    }
}
