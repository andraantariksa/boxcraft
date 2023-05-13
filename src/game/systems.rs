// use crate::game::camera::Camera;
// use parking_lot::Mutex;
// use std::sync::Arc;
//
// use crate::game::player::Player;
// use crate::misc::input::InputManager;
//
// use crate::boxworld::World as WorldEnvironment;
//
// use crate::game::physics::Physics;
// // use crate::game::physics::Physics;
// use legion::{Resources, Schedule, World};
// use rapier3d::dynamics::RigidBodyType;
// use rapier3d::prelude::{RigidBody, RigidBodyBuilder};
// use std::time::Duration;
//
// pub struct Systems {
//
// }
//
// impl Systems {
//     pub fn new(input_manager: InputManager, camera: Camera) -> Self {
//         let mut resources = Resources::default();
//         let mut world = World::default();
//         let player = Player::new();
//
//         resources.insert(WorldEnvironment::from(&camera));
//         resources.insert(camera);
//         resources.insert(input_manager);
//
//         // let mut physics = Physics::new();
//
//         // let rb = RigidBodyBuilder::new(RigidBodyType::KinematicVelocityBased)
//         //     .gravity_scale(0.0)
//         //     .build();
//         // let rb_handle = physics.rigid_body_set.insert(rb);
//
//         // world.push((player, rb_handle));
//
//         Self {
//             schedule: Schedule::builder()
//                 .add_system(crate::game::player::update_player_system())
//                 // .add_system(crate::game::physics::update_physics_system())
//                 .build(),
//             world,
//             resources,
//             // physics,
//         }
//     }
//
//     pub fn update(&mut self, elapsed_time: Duration) {
//         self.resources.insert(elapsed_time);
//         self.schedule.execute(&mut self.world, &mut self.resources);
//     }
//
//     pub fn get_resources(&self) -> &Resources {
//         &self.resources
//     }
// }
//
// pub trait InitSystem {
//     fn init(resources: &mut Resources);
// }

use crate::utils::time::get_timestamp;
use bevy_ecs::prelude::*;
use std::time::Duration;

#[derive(Resource)]
pub struct Time {
    pub dt: f32,
    pub start: f32,
}

impl Time {
    pub fn new() -> Self {
        Self {
            dt: 0.0,
            start: get_timestamp(),
        }
    }
}

impl From<f32> for Time {
    fn from(value: f32) -> Self {
        Self {
            dt: value,
            start: get_timestamp(),
        }
    }
}

impl From<Duration> for Time {
    fn from(value: Duration) -> Self {
        Self {
            dt: value.as_secs_f32(),
            start: get_timestamp(),
        }
    }
}
