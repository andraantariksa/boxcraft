pub mod camera;
pub mod config;
pub mod debug_ui;
pub mod physics;
pub mod player;
pub mod systems;
pub mod transform;
pub mod world;

use crate::debug_ui::DebugUI;
use crate::game::camera::Camera;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;

use crate::misc::input::InputManager;
// use crate::game::physics::Physics;
use crate::misc::window::Window;
use crate::renderer::Renderer;
use bevy_ecs::prelude::*;

use bevy_ecs::system::SystemState;
use parking_lot::Mutex;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::game::player::{update_player, update_player_toggle_fly, Player};
use crate::game::systems::Time;
use crate::game::world::chunk::Chunk;
use crate::game::world::BoxWorld;
use crate::utils::time::get_timestamp;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::CursorGrabMode;

pub struct Game {
    event_loop: EventLoop<()>,
    window: Window,

    debug_ui: DebugUI,
    is_cursor_locked: bool,

    renderer: Renderer,
    world: World,
    schedule: Schedule,

    chunk_rx: Receiver<Chunk>,
    to_world_tx: Sender<Chunk>,
}

impl Game {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop);

        let mut debug_ui = DebugUI::new(&*window);

        let mut world = World::new();
        let camera = Camera::new();
        world.insert_resource(BoxWorld::from(&camera));
        world.insert_resource(camera);
        world.insert_resource(InputManager::new());
        world.insert_resource(Player::new());
        world.insert_resource(Time::new());

        let camera = world.get_resource::<Camera>().unwrap();
        let renderer = pollster::block_on(Renderer::new(&window, &camera, &mut debug_ui));

        let mut schedule = Schedule::new();
        schedule
            .add_system(update_player)
            .add_system(update_player_toggle_fly);

        log::info!("Main thread {:?}", std::thread::current().id());

        let (to_world_tx, chunk_rx) = channel();

        Self {
            event_loop,
            debug_ui,
            window,
            renderer,
            world,
            schedule,
            is_cursor_locked: true,
            chunk_rx,
            to_world_tx,
        }
    }

    pub fn run_loop(mut self) {
        {
            let mut world_blocks = self.world.get_resource::<BoxWorld>().unwrap();
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
                        self.window.on_resized(new_inner_size);
                        self.renderer.resize(new_inner_size);
                    }
                    WindowEvent::ScaleFactorChanged {
                        scale_factor,
                        new_inner_size,
                    } => {
                        self.window.on_resized(&new_inner_size);
                        self.renderer.resize(*new_inner_size);
                    }
                    rest_window_event => {
                        let mut state =
                            SystemState::<(ResMut<InputManager>, Res<Time>)>::new(&mut self.world);
                        let (mut input_manager, elapsed_time) = state.get_mut(&mut self.world);
                        input_manager.record_event(
                            &self.window,
                            rest_window_event,
                            self.is_cursor_locked,
                            elapsed_time.stamp,
                        );
                    }
                },
                Event::MainEventsCleared => {
                    let time_elapsed = time_start.elapsed();
                    time_start = Instant::now();

                    self.world.insert_resource(Time::from(time_elapsed));

                    self.schedule.run(&mut self.world);
                    self.debug_ui
                        .update(&self.world, &self.window, &time_elapsed);
                    let debug_ui_render_data = self.debug_ui.get_draw_data(&self.window);

                    let debug_ui_render_state =
                        self.debug_ui
                            .update(&self.world, &self.window, &time_elapsed);

                    let mut state = SystemState::<(
                        ResMut<InputManager>,
                        ResMut<Camera>,
                        ResMut<BoxWorld>,
                    )>::new(&mut self.world);
                    let (mut input_manager, mut camera, mut world) = state.get_mut(&mut self.world);

                    let mouse_movement = input_manager.get_mouse_movement();
                    camera.move_by_offset(&mouse_movement, &time_elapsed);

                    let is_f2_pressed = input_manager.is_key_pressed(&VirtualKeyCode::F2);
                    if is_f2_pressed {
                        self.renderer.game_renderer.set_display_wireframe_only(
                            &self.renderer.render_context,
                            !self.renderer.game_renderer.is_wireframe_only(),
                        );
                    };

                    if world.update_chunk(&self.to_world_tx, &self.chunk_rx, &camera) {
                        let block_raw_instances = world.get_block_raw_instances();
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
                        &world,
                    );

                    input_manager.clear();
                }
                _ => {}
            }
        });
    }
}
