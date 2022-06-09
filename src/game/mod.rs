pub mod debug_ui;
pub mod player;
pub mod systems;
pub mod world;

use crate::game::debug_ui::DebugUI;
use crate::game::player::Player;
use crate::game::systems::Systems;
use crate::physics::Physics;
use crate::{InputManager, Renderer};
use std::rc::Rc;
use std::time::Instant;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

pub struct Game {
    event_loop: EventLoop<()>,
    input_manager: InputManager,
    window: Rc<Window>,

    debug_ui: DebugUI,

    renderer: Renderer,
    physics: Physics,
    systems: Systems,
}

impl Game {
    pub(crate) fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = Rc::new(WindowBuilder::new().build(&event_loop).unwrap());

        let input_manager = InputManager::new(Rc::clone(&window));
        let mut debug_ui = DebugUI::new(&*window);
        let renderer = pollster::block_on(Renderer::new(Rc::clone(&window), &mut debug_ui));

        Self {
            event_loop,
            debug_ui,
            renderer,
            input_manager,
            window,
            systems: Systems::new(),
            physics: Physics::new(),
        }
    }

    pub(crate) fn run_loop(mut self) {
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
                    rest_window_event => {
                        self.input_manager.record_event(&rest_window_event);
                    }
                },
                Event::MainEventsCleared => {
                    let time_elapsed = time_start.elapsed();
                    time_start = Instant::now();

                    self.systems.update();

                    let debug_ui_render_state =
                        self.debug_ui
                            .update(&self.systems.world, &self.window, &time_elapsed);

                    self.renderer.render(&debug_ui_render_state, &time_elapsed);
                }
                _ => {}
            }
        });
    }
}
