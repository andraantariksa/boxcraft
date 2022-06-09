use crate::game::debug_ui::DebugUI;
use crate::game::player::Player;
use crate::renderer::context::RenderContext;
use legion::{Resources, Schedule, World, WorldOptions};

pub struct Systems {
    schedule: Schedule,
    pub(crate) world: World,
    resources: Resources,

    player: Player,
}

impl Systems {
    pub fn new() -> Self {
        Self {
            schedule: Schedule::builder().build(),
            world: World::default(),
            resources: Resources::default(),
            player: Player::new(),
        }
    }

    pub fn update(&mut self) {
        self.schedule.execute(&mut self.world, &mut self.resources);
    }
}
