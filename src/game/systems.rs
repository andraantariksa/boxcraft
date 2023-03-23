use crate::game::camera::Camera;
use parking_lot::Mutex;
use std::sync::Arc;

use crate::game::player::Player;
use crate::misc::input::InputManager;

use crate::game::world::World as WorldEnvironment;

use legion::{Resources, Schedule, World};
use std::time::Duration;
use rapier3d::dynamics::RigidBodyType;
use rapier3d::prelude::{RigidBody, RigidBodyBuilder};
use crate::misc::physics::Physics;

pub struct Systems {
    schedule: Schedule,
    pub world: World,
    pub resources: Resources,
    physics: Physics
}

impl Systems {
    pub fn new(input_manager: InputManager, camera: Camera) -> Self {
        let mut resources = Resources::default();
        let mut world = World::default();
        let player = Player::new();

        resources.insert(WorldEnvironment::from(&camera));
        resources.insert(camera);
        resources.insert(input_manager);

        let mut physics = Physics::new();

        let rb = RigidBodyBuilder::new(RigidBodyType::KinematicVelocityBased).gravity_scale(0.0).build();
        let rb_handle = physics.rigid_body_set.insert(rb);

        world.push((player, rb_handle));

        Self {
            schedule: Schedule::builder()
                .add_system(crate::game::player::update_player_system())
                .build(),
            world,
            resources,
            physics
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

#[system(for_each)]
pub fn update_physics() {

}
