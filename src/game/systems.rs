use crate::game::camera::Camera;
use crate::game::debug_ui::DebugUI;
use crate::game::player::Player;
use crate::renderer::context::RenderContext;
use crate::InputManager;
use atomic_refcell::{AtomicRef, AtomicRefMut};
use legion::{IntoQuery, Resources, Schedule, World, WorldOptions};
use std::time::Duration;

pub struct Systems {
    schedule: Schedule,
    pub world: World,
    pub resources: Resources,
}

impl Systems {
    pub fn new(input_manager: InputManager, camera: Camera) -> Self {
        let mut resources = Resources::default();
        resources.insert(camera);
        resources.insert(input_manager);

        let mut world = World::default();
        world.push((Player::new(),));

        Self {
            schedule: Schedule::builder()
                .add_system(crate::game::player::update_player_system())
                .build(),
            world,
            resources,
        }
    }

    pub fn update(&mut self, elapsed_time: Duration) {
        self.resources.insert(elapsed_time);
        self.schedule.execute(&mut self.world, &mut self.resources);
    }

    pub fn get_camera_mut(&self) -> AtomicRefMut<Camera> {
        self.resources.get_mut::<Camera>().unwrap()
    }
    pub fn get_input_manager_mut(&self) -> AtomicRefMut<InputManager> {
        self.resources.get_mut::<InputManager>().unwrap()
    }
}
