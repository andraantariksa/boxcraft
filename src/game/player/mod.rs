use crate::game::camera::Camera;

use crate::app::input::InputManager;

use crate::game::systems::Time;
use crate::physic::Physics;
use crate::plugin::Plugin;
use bevy_ecs::prelude::*;
use rapier3d::prelude::*;
use winit::event::VirtualKeyCode;
use winit::window::Window;

#[derive(Resource)]
pub struct Player {
    pub flying: bool,
    pub rb_handle: RigidBodyHandle,
}

impl Player {
    pub fn from(rb_handle: RigidBodyHandle) -> Self {
        Self {
            flying: true,
            rb_handle,
        }
    }
}

pub fn update_player_toggle_fly(
    mut player: ResMut<Player>,
    input_manager: Res<InputManager>,
    mut physics: ResMut<Physics>,
) {
    if input_manager.is_double_pressed(&VirtualKeyCode::Space) {
        player.flying = !player.flying;

        let rb = physics.rigid_body_set.get_mut(player.rb_handle).unwrap();

        let scale = if player.flying { 0.0 } else { 1.0 };
        rb.set_gravity_scale(scale, false);

        if player.flying {
            rb.set_linvel(Vector::new(0.0, 0.0, 0.0), false);
        }
    }
}

pub fn update_player(
    player: Res<Player>,
    camera: ResMut<Camera>,
    input_manager: Res<InputManager>,
    elapsed_time: Res<Time>,
    mut physics: ResMut<Physics>,
) {
    const SPEED_MOVEMENT: f32 = 100.0;

    let rb = physics.rigid_body_set.get_mut(player.rb_handle).unwrap();
    let mut translation = *rb.translation();

    let delta_movement = SPEED_MOVEMENT * elapsed_time.dt;
    let right_direction = camera.get_direction_right_horizontally();
    let horizontal_direction = camera.get_direction_horizontally();

    if input_manager.is_key_pressed(&VirtualKeyCode::A) {
        translation -= delta_movement * right_direction;
    } else if input_manager.is_key_pressed(&VirtualKeyCode::D) {
        translation += delta_movement * right_direction;
    }

    if input_manager.is_key_pressed(&VirtualKeyCode::W) {
        translation += delta_movement * horizontal_direction;
    } else if input_manager.is_key_pressed(&VirtualKeyCode::S) {
        translation -= delta_movement * horizontal_direction;
    }

    if input_manager.is_key_pressed(&VirtualKeyCode::Space) {
        translation += delta_movement * Camera::WORLD_UP;
    } else if input_manager.is_key_pressed(&VirtualKeyCode::LControl) {
        translation -= delta_movement * Camera::WORLD_UP;
    }

    rb.set_translation(translation, false);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn register_init(&self, _world: &mut World, init_schedule: &mut Schedule, _window: &Window) {
        pub fn init_player(
            mut commands: Commands,
            mut physics: ResMut<Physics>,
            camera: Res<Camera>,
        ) {
            let physics = &mut *physics;

            let rb = RigidBodyBuilder::dynamic()
                .position(Isometry::from(camera.position))
                .gravity_scale(0.0)
                .build();
            let rb_handle = physics.rigid_body_set.insert(rb);

            let col = ColliderBuilder::ball(1.0);
            physics
                .collider_set
                .insert_with_parent(col, rb_handle, &mut physics.rigid_body_set);

            commands.insert_resource(Player::from(rb_handle));
        }

        init_schedule.add_systems(init_player);
    }

    fn register_runtime(&self, _world: &mut World, schedule: &mut Schedule) {
        pub fn update_player_physics(
            player: Res<Player>,
            mut physics: ResMut<Physics>,
            mut camera: ResMut<Camera>,
        ) {
            let rb = physics.rigid_body_set.get_mut(player.rb_handle).unwrap();
            camera.position = Point::from(*rb.translation());
        }

        schedule.add_systems(update_player_physics);
    }
}
