use crate::boxworld::BoxWorld;
use crate::game::camera::Camera;
use bevy_ecs::prelude::*;

pub fn sync_camera(camera: Res<Camera>, mut box_world: ResMut<BoxWorld>) {
    box_world.set_camera(&*camera);
}
