pub mod camera;
pub mod components;
pub mod config;
pub mod player;
pub mod systems;
pub mod world;

use crate::game::camera::Camera;
use crate::ui::{update_draw_ui, UI};
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::misc::input::InputManager;
use crate::misc::window::Window;
use crate::renderer::Renderer;
use bevy_ecs::prelude::*;

use bevy_ecs::system::SystemState;

use std::time::Instant;

use crate::game::player::{update_player, update_player_toggle_fly, Player};
use crate::game::systems::Time;
use crate::game::world::chunk::Chunk;
use crate::game::world::BoxWorld;
use crate::physic::Physics;

use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub struct Game {
    event_loop: Option<EventLoop<()>>,
    window: Window,
    is_cursor_locked: bool,

    renderer: Renderer,
    world: World,
    schedule: Schedule,

    chunk_rx: Receiver<Chunk>,
    to_world_tx: Sender<Chunk>,
    time_start: Instant,
}

impl Game {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop);

        let mut world = World::new();
        let camera = Camera::new();
        let mut ui = UI::new(&window);

        let renderer = pollster::block_on(Renderer::new(&window, &camera, &mut ui));

        world.insert_resource(BoxWorld::from(&camera));
        world.insert_resource(camera);
        world.insert_resource(InputManager::new());
        world.insert_resource(Player::new());
        world.insert_resource(Time::new());
        world.insert_resource(ui);
        world.insert_resource(Physics::new());

        let mut schedule = Schedule::new();
        schedule
            .add_system(update_player)
            .add_system(update_player_toggle_fly)
            .add_system(update_draw_ui);

        log::info!("Main thread {:?}", std::thread::current().id());

        let (to_world_tx, chunk_rx) = channel();

        Self {
            event_loop: Some(event_loop),
            window,
            renderer,
            world,
            schedule,
            is_cursor_locked: true,
            chunk_rx,
            to_world_tx,
            time_start: Instant::now(),
        }
    }

    pub fn run_loop(mut self) {
        {
            let world_blocks = self.world.get_resource::<BoxWorld>().unwrap();
            let block_raw_instances = world_blocks.get_block_raw_instances();
            self.renderer.game_renderer.update_blocks(
                &self.renderer.render_context,
                block_raw_instances,
                block_raw_instances.len() as u32,
            );
        }

        self.time_start = Instant::now();
        self.event_loop
            .take()
            .unwrap()
            .run(move |event, _, control_flow| {
                {
                    let mut ui = self.world.get_resource_mut::<UI>().unwrap();
                    ui.record_event(&event);
                }
                match event {
                    Event::WindowEvent {
                        event: ref window_event,
                        window_id,
                    } if window_id == self.window.id() => {
                        self.process_window_event(window_event, control_flow)
                    }
                    Event::MainEventsCleared => {
                        self.update();
                    }
                    _ => {}
                }
            });
    }

    fn process_window_event(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) {
        match event {
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
            WindowEvent::Resized(ref new_inner_size) => {
                self.window.on_resized(new_inner_size);
                self.renderer.resize(new_inner_size);
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor: _,
                new_inner_size,
            } => {
                self.window.on_resized(new_inner_size);
                self.renderer.resize(new_inner_size);
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
        }
    }

    fn update(&mut self) {
        let time_elapsed = self.time_start.elapsed();
        self.time_start = Instant::now();

        self.world.insert_resource(Time::from(time_elapsed));

        {
            let mut ui = self.world.get_resource_mut::<UI>().unwrap();
            ui.pre_update(time_elapsed.as_secs_f64());
        }
        self.schedule.run(&mut self.world);

        let mut state = SystemState::<(
            ResMut<InputManager>,
            ResMut<Camera>,
            ResMut<BoxWorld>,
            ResMut<UI>,
        )>::new(&mut self.world);
        let (mut input_manager, mut camera, mut world, mut ui) = state.get_mut(&mut self.world);

        let ui_render_data = ui.get_draw_data(&self.window);

        let mouse_movement = input_manager.get_mouse_movement();
        camera.move_by_offset(mouse_movement, &time_elapsed);

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
                block_raw_instances,
                block_raw_instances.len() as u32,
            );
        }
        self.renderer
            .render(&camera, &time_elapsed, &self.window, ui_render_data, &world);

        input_manager.clear();
    }
}
