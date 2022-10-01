use crate::game::camera::Camera;

use crate::game::player::Player;
use crate::misc::input::InputManager;

use crate::game::world::World as WorldEnvironment;
use atomic_refcell::AtomicRefMut;
use legion::{Resources, Schedule, World};
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
        let player = Player::new();

        resources.insert(WorldEnvironment::from(&player));

        world.push((player,));

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

    pub fn get_resources(&self) -> &Resources {
        &self.resources
    }
}
