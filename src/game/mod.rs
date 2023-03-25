pub mod camera;
pub mod player;
pub mod systems;
pub mod transform;
pub mod world;

use crate::debug_ui::DebugUI;
use crate::game::camera::Camera;
use std::sync::Arc;

use crate::game::systems::Systems;
use crate::misc::input::InputManager;
use crate::misc::physics::Physics;
use crate::misc::window::Window;
use crate::renderer::Renderer;

use parking_lot::Mutex;
use std::time::Instant;

use crate::game::world::World;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::CursorGrabMode;

pub struct Game {
    event_loop: EventLoop<()>,
    window: Window,

    debug_ui: DebugUI,
    is_cursor_locked: bool,

    renderer: Renderer,
    physics: Physics,
    systems: Systems,
}

impl Game {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop);

        let mut debug_ui = DebugUI::new(&*window);

        let systems = Systems::new(InputManager::new(), Camera::new());
        let camera = systems.get_resources().get::<Camera>().unwrap();
        let renderer = pollster::block_on(Renderer::new(&window, &camera, &mut debug_ui));
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
        {
            let mut world_blocks = self.systems.get_resources().get_mut::<World>().unwrap();
            let block_raw_instances = world_blocks.get_block_raw_instances();
            self.renderer.game_renderer.update_blocks(
                &self.renderer.render_context,
                &block_raw_instances,
                block_raw_instances.len() as u32,
            );
        }

        let mut time_start = Instant::now();
        self.event_loop.run(move |event, _, control_flow| {
            self.debug_ui.record_event(&event);
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
                        // self.window.set_cursor_grab(CursorGrabMode::Locked).unwrap();
                    }
                    WindowEvent::Resized(new_inner_size) => {
                        self.window.on_resized(&new_inner_size);
                        self.renderer.resize(&new_inner_size);
                    }
                    WindowEvent::ScaleFactorChanged {
                        scale_factor,
                        new_inner_size,
                    } => {
                        self.window.on_resized(&new_inner_size);
                        self.renderer.resize(*new_inner_size);
                    }
                    rest_window_event => {
                        let mut input_manager = self
                            .systems
                            .get_resources()
                            .get_mut::<InputManager>()
                            .unwrap();
                        input_manager.record_event(
                            &self.window,
                            rest_window_event,
                            self.is_cursor_locked,
                        );
                    }
                },
                Event::MainEventsCleared => {
                    let time_elapsed = time_start.elapsed();
                    time_start = Instant::now();

                    self.systems.update(time_elapsed);

                    self.debug_ui.update(
                        &self.systems.world,
                        &self.systems.resources,
                        &self.window,
                        &time_elapsed,
                    );
                    let debug_ui_render_data = self.debug_ui.get_draw_data(&self.window);

                    let mut input_manager = self
                        .systems
                        .get_resources()
                        .get_mut::<InputManager>()
                        .unwrap();

                    let mut camera = self.systems.get_resources().get_mut::<Camera>().unwrap();
                    camera.move_by_offset(input_manager.get_mouse_movement(), &time_elapsed);

                    let is_f2_pressed = input_manager.is_key_pressed(&VirtualKeyCode::F2);
                    if is_f2_pressed {
                        self.renderer.game_renderer.set_display_wireframe_only(
                            &self.renderer.render_context,
                            !self.renderer.game_renderer.is_wireframe_only(),
                        );
                    };

                    let mut world_blocks = self.systems.get_resources().get_mut::<World>().unwrap();
                    if world_blocks.update_chunk(&camera) {
                        let block_raw_instances = world_blocks.get_block_raw_instances();
                        self.renderer.game_renderer.update_blocks(
                            &self.renderer.render_context,
                            &block_raw_instances,
                            block_raw_instances.len() as u32,
                        );
                    }
                    self.renderer.render(
                        &*camera,
                        &time_elapsed,
                        &self.window,
                        debug_ui_render_data,
                        &world_blocks,
                    );

                    input_manager.clear();
                }
                _ => {}
            }
        });
    }
}
