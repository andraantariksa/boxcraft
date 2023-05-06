use rapier3d::prelude::*;

use crate::plugin::Plugin;
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct Physics {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub physics_pipeline: PhysicsPipeline,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
    gravity: Vector<Real>,
}

impl Physics {
    pub fn new() -> Self {
        Self {
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            physics_pipeline: PhysicsPipeline::new(),
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            gravity: Vector::new(0.0, -9.81, 0.0),
            event_handler: (),
            physics_hooks: (),
            ccd_solver: Default::default(),
        }
    }

    pub fn update(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            None,
            &self.physics_hooks,
            &self.event_handler,
        );
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn register_init(&self, world: &mut World, init_schedule: &mut Schedule) {
        world.insert_resource(Physics::new());
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
