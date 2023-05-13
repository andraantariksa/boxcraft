use crate::boxworld::chunk::Chunk;
use crate::boxworld::worker::BoxWorldTask;
use crate::boxworld::BoxWorld;
use crate::game::camera::Camera;
use crate::renderer::game_renderer::GameRenderer;
use crate::renderer::Renderer;
use crate::worker::ComputeTaskPool;
use bevy_ecs::prelude::*;
use futures_lite::future;
use nalgebra::Vector2;
use std::collections::HashMap;
use std::ops::Deref;

pub fn update_worker(
    mut commands: Commands,
    mut query: Query<(Entity, &mut BoxWorldTask)>,
    mut box_world: ResMut<BoxWorld>,
) {
    for (entity, mut task) in query.iter_mut() {
        let maybe_result = future::block_on(future::poll_once(&mut task.0));
        if let Some(result) = maybe_result {
            box_world.insert_chunk(result.coord, result.chunk);
            commands.entity(entity).despawn();
        }
    }
}

pub fn calculate(
    commands: Commands,
    mut world: ResMut<BoxWorld>,
    renderer: Res<Renderer>,
    camera: Res<Camera>,
    mut task_pool: ResMut<ComputeTaskPool>,
    mut game_renderer: ResMut<GameRenderer>,
) {
    if world.update_current_chunk_coord(&camera) {
        // Enqueue calculation
        println!("Enqueued");
        world.enqueue_work(&mut *task_pool, commands);
    }

    if world.is_dirty() {
        println!("Dirty!");
        world.update_blocks(&renderer, &mut game_renderer);
    }
}
