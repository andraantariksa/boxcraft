use crate::game::schedule::ScheduleStage;
use crate::physic::update_physics;
use crate::physic::Physics;
use crate::plugin::Plugin;
use bevy_ecs::prelude::*;
use winit::window::Window;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn register_init(&self, world: &mut World, _init_schedule: &mut Schedule, _window: &Window) {
        world.insert_resource(Physics::new());
    }

    fn register_runtime(&self, _world: &mut World, schedule: &mut Schedule) {
        schedule.add_systems(update_physics.in_set(ScheduleStage::PostUpdate));
    }
}

// pub struct PhysicsCommand {
//     deleted_handle: Vec<RigidBodyHandle>,
// }
//
// impl PhysicsCommand {
//     pub fn new() -> Self {
//         Self {
//             deleted_handle: Vec::new(),
//         }
//     }
// }
//
// pub fn init_physics(world: &mut World) {
//     world.insert_resource(PhysicsCommand::new());
// }
//
// pub fn process_command(command: Res<PhysicsCommand>, mut physics: ResMut<Physics>) {
//     for handle in command.deleted_handle {
//         physics
//             .rigid_body_set
//             .remove(
//                 handle,
//                 &mut physics.island_manager,
//                 &mut physics.collider_set,
//                 &mut physics.impulse_joint_set,
//                 &mut physics.multibody_joint_set,
//                 true,
//             )
//             .unwrap();
//     }
//     command.deleted_handle.clear();
// }
