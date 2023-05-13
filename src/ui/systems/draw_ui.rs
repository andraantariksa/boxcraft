use crate::game::camera::Camera;
use crate::game::player::Player;
use crate::ui::UI;
use bevy_ecs::prelude::*;
use egui::{Color32, Style, Visuals};

pub fn draw_ui(ui: Res<UI>, player: Res<Player>, camera: Res<Camera>) {
    let ctx = ui.context();

    let (camera_yaw, camera_pitch) = camera.get_yaw_pitch();

    ctx.set_style(Style {
        visuals: Visuals {
            window_fill: Color32::from_black_alpha(20),
            ..Default::default()
        },
        ..Default::default()
    });

    // self.resp.push_back(time_elapsed);

    egui::Window::new("Inspector")
        .resizable(false)
        .default_width(300.0)
        .show(&ctx, |ui| {
            egui::trace!(ui);
            ui.vertical_centered(|ui| {
                ui.heading("Inspect");
            });

            ui.separator();

            ui.label(format!("Yaw: {:.1} Pitch: {:.1}", camera_yaw, camera_pitch));
            ui.label(format!("Direction: {}", camera.get_direction()));
            ui.label(format!("Pos: {}", camera.position));
            let mut a = player.flying;
            ui.checkbox(&mut a, "Fly");

            ui.separator();

            // Plot::new("FPS").show(ui, |plot_ui| {
            //     let l = self
            //         .resp
            //         .iter()
            //         .enumerate()
            //         .map(|(a, b)| [a.to_f64(), *b])
            //         .collect::<PlotPoints>();
            //     plot_ui.line(Line::new(l));
            // })
        });
}
