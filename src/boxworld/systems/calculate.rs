use crate::game::camera::Camera;

use crate::boxworld::chunk::Chunk;
use crate::boxworld::BoxWorld;
use bevy_ecs::prelude::*;

use bevy_tasks::Task;
use futures_lite::future;

#[derive(Component)]
pub struct ChunkCalculationTask(pub Task<Chunk>);

pub fn poll_calculation_task(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ChunkCalculationTask)>,
) {
    for (e, mut task) in tasks.iter_mut() {
        let res = future::block_on(future::poll_once(&mut task.0));
        if res.is_some() {
            commands.entity(e).despawn();
        }
    }
}

pub fn calculate(
    commands: Commands,
    mut world: ResMut<BoxWorld>,
    // renderer: Res<Renderer>,
    camera: Res<Camera>,
    // mut game_renderer: ResMut<GameRenderer>,
) {
    if world.update_current_chunk_coord(&camera) {
        // Enqueue calculation
    }

    // println!("Updating");
    if world.is_dirty() {
        println!("Updating");
        world.try_enqueue_work(commands);
        // world.update_blocks(&renderer, &mut game_renderer);
    }
}
