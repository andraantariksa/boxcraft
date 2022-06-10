use crate::game::camera::Camera;
use crate::game::debug_ui::DebugUI;
use crate::game::player::Player;
use crate::renderer::context::RenderContext;
use atomic_refcell::{AtomicRef, AtomicRefMut};
use legion::{IntoQuery, Resources, Schedule, World, WorldOptions};
use std::time::Duration;

pub struct Systems {
    schedule: Schedule,
    pub(crate) world: World,
    resources: Resources,
}

impl Systems {
    pub fn new() -> Self {
        let mut resources = Resources::default();
        resources.insert(Camera::new());

        let mut world = World::default();
        world.push((Player::new(),));

        Self {
            schedule: Schedule::builder().build(),
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
}
