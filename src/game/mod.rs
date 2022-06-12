pub mod block;
pub mod camera;
pub mod debug_ui;
pub mod player;
pub mod systems;
pub mod transform;
pub mod world;

use crate::game::camera::Camera;
use crate::game::debug_ui::DebugUI;
use crate::game::player::Player;
use crate::game::systems::Systems;
use crate::physics::Physics;
use crate::renderer::Renderer;
use crate::InputManager;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Instant;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

pub struct Game {
    event_loop: EventLoop<()>,
    window: Rc<Window>,

    debug_ui: DebugUI,
    is_cursor_locked: bool,

    renderer: Renderer,
    physics: Physics,
    systems: Systems,
}

impl Game {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = Rc::new(
            WindowBuilder::new()
                .with_inner_size(PhysicalSize {
                    width: 1024,
                    height: 768,
                })
                .build(&event_loop)
                .unwrap(),
        );

        let mut debug_ui = DebugUI::new(&*window);

        let systems = Systems::new(InputManager::new(), Camera::new());
        let camera = systems.get_camera_mut();
        let renderer =
            pollster::block_on(Renderer::new(Rc::clone(&window), &camera, &mut debug_ui));
        drop(camera);

        Self {
            event_loop,
            debug_ui,
            window,
            renderer,
            systems,
            is_cursor_locked: true,
            physics: Physics::new(),
        }
    }

    pub fn run_loop(mut self) {
        let mut time_start = Instant::now();
        self.event_loop.run(move |event, _, control_flow| {
            self.debug_ui.record_event(&self.window, &event);
            match event {
                Event::WindowEvent {
                    event: ref window_event,
                    window_id,
                } if window_id == self.window.id() => match window_event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::F1),
                                ..
                            },
                        ..
                    } => {
                        self.is_cursor_locked = !self.is_cursor_locked;
                        self.window.set_cursor_grab(self.is_cursor_locked).unwrap();
                    }
                    WindowEvent::Resized(_) => {
                        let new_window_size = self.window.inner_size();
                        self.renderer.resize(&new_window_size);
                    }
                    rest_window_event => {
                        let mut input_manager = self.systems.get_input_manager_mut();
                        input_manager.record_event(
                            &self.window,
                            &rest_window_event,
                            self.is_cursor_locked,
                        );
                    }
                },
                Event::MainEventsCleared => {
                    let time_elapsed = time_start.elapsed();
                    time_start = Instant::now();

                    self.systems.update(time_elapsed.clone());

                    let debug_ui_render_state = self.debug_ui.update(
                        &self.systems.world,
                        &self.systems.resources,
                        &self.window,
                        &time_elapsed,
                    );

                    let mut input_manager = self.systems.get_input_manager_mut();

                    let mut camera = self.systems.get_camera_mut();
                    camera.move_by_offset(input_manager.get_mouse_movement(), &time_elapsed);

                    let is_f2_pressed = input_manager.is_key_pressed(&VirtualKeyCode::F2);
                    if is_f2_pressed {
                        self.renderer.game_renderer.set_display_wireframe_only(
                            &self.renderer.render_context,
                            !self.renderer.game_renderer.is_wireframe_only(),
                        );
                    }

                    self.renderer
                        .render(&*camera, &time_elapsed, &debug_ui_render_state);

                    input_manager.clear();
                }
                _ => {}
            }
        });
    }
}
