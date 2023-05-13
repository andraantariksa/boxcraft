use crate::game::systems::Time;
use crate::ui::UI;
use bevy_ecs::prelude::*;

pub fn pre_update(mut ui: ResMut<UI>, time: Res<Time>) {
    ui.pre_update(time.dt as f64);
}
