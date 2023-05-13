use bevy_ecs::prelude::*;

#[derive(Debug, Eq, PartialEq, SystemSet, Hash, Clone)]
pub enum ScheduleStage {
    PreUpdate,
    Update,
    PostUpdate,
    PreRender,
    Render,
}
