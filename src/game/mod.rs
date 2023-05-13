pub mod camera;
pub mod common;
pub mod config;
pub mod player;
pub mod schedule;
pub mod systems;

use crate::game::camera::{Camera, CameraPlugin};
use crate::ui::UI;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::app::input::InputManager;
use crate::misc::window::Window;
use crate::renderer::Renderer;
use bevy_ecs::prelude::*;
use bevy_ecs::schedule::SystemSet;
use futures_lite::future;

use bevy_ecs::system::SystemState;
use std::time::Instant;

use crate::boxworld::chunk::Chunk;
use crate::boxworld::BoxWorld;
use crate::game::player::{update_player, update_player_toggle_fly, PlayerPlugin};
use crate::game::systems::Time;
use crate::physic::Physics;

use crate::app::input::plugin::InputPlugin;
use crate::boxworld::plugin::WorldPlugin;
use crate::physic::plugin::PhysicsPlugin;
use crate::plugin::Plugin;
use crate::renderer::game_renderer::GameRenderer;
use crate::renderer::plugins::RendererPlugin;
use crate::ui::plugin::UIPlugin;
use crate::worker::plugins::WorkerPlugin;
use schedule::ScheduleStage;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub struct Game {
    event_loop: Option<EventLoop<()>>,
    window: Window,
    is_cursor_locked: bool,

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

        let plugins: &[&dyn Plugin] = &[
            &InputPlugin,
            &PhysicsPlugin,
            &CameraPlugin,
            &PlayerPlugin,
            &RendererPlugin,
            &WorldPlugin,
            &UIPlugin,
            &WorkerPlugin,
        ];
        let mut init_schedule = Schedule::new();
        for plugin in plugins.iter() {
            plugin.register_init(&mut world, &mut init_schedule, &window);
        }
        init_schedule.run(&mut world);

        let camera = world.get_resource::<Camera>().unwrap();
        let r = future::block_on(Renderer::new(&window));
        let gr = GameRenderer::new(&r.render_context, &window, camera);
        world.insert_resource(gr);
        world.insert_resource(r);
        world.insert_resource(Time::new());

        let mut schedule = Schedule::new();
        for plugin in plugins.iter() {
            plugin.register_runtime(&mut world, &mut schedule);
        }
        schedule
            .add_system(update_player)
            .add_system(update_player_toggle_fly)
            .set_default_base_set(ScheduleStage::Update)
            .configure_sets(
                (
                    ScheduleStage::PreUpdate,
                    ScheduleStage::Update,
                    ScheduleStage::PostUpdate,
                    ScheduleStage::PreRender,
                    ScheduleStage::Render,
                )
                    .chain(),
            );

        log::info!("Main thread {:?}", std::thread::current().id());

        let (to_world_tx, chunk_rx) = channel();

        Self {
            event_loop: Some(event_loop),
            window,
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
            // let mut state =
            //     SystemState::<(ResMut<BoxWorld>, Res<Renderer>, ResMut<GameRenderer>)>::new(
            //         &mut self.world,
            //     );
            // let (mut world_blocks, renderer, mut game_renderer) = state.get_mut(&mut self.world);
            //
            // let block_raw_instances = world_blocks.get_raw_face_instances();
            // game_renderer.update_blocks(
            //     &renderer.render_context,
            //     block_raw_instances,
            //     world_blocks.get_block_count() as u32,
            // );
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
                let mut renderer = self.world.get_resource_mut::<Renderer>().unwrap();
                renderer.resize(new_inner_size);
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor: _,
                new_inner_size,
            } => {
                self.window.on_resized(new_inner_size);
                let mut renderer = self.world.get_resource_mut::<Renderer>().unwrap();
                renderer.resize(new_inner_size);
            }
            rest_window_event => {
                let mut state =
                    SystemState::<(ResMut<InputManager>, Res<Time>)>::new(&mut self.world);
                let (mut input_manager, elapsed_time) = state.get_mut(&mut self.world);
                input_manager.record_event(
                    &self.window,
                    rest_window_event,
                    self.is_cursor_locked,
                    elapsed_time.start,
                );
            }
        }
    }

    fn update(&mut self) {
        let time_elapsed = self.time_start.elapsed();
        self.time_start = Instant::now();

        self.world.insert_resource(Time::from(time_elapsed));

        self.schedule.run(&mut self.world);

        let mut state = SystemState::<(
            ResMut<Camera>,
            Res<GameRenderer>,
            ResMut<UI>,
            ResMut<Renderer>,
        )>::new(&mut self.world);
        let (mut camera, game_renderer, mut ui, mut renderer) = state.get_mut(&mut self.world);

        let ui_render_data = ui.get_draw_data(&self.window);
        renderer.render(&camera, &self.window, ui_render_data, &game_renderer);
    }
}
