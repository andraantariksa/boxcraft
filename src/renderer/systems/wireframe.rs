use crate::app::input::InputManager;
use crate::renderer::game_renderer::GameRenderer;
use crate::renderer::Renderer;
use bevy_ecs::system::ResMut;
use winit::event::VirtualKeyCode;

pub fn update_switch_wireframe(
    mut game_renderer: ResMut<GameRenderer>,
    renderer: ResMut<Renderer>,
    input_manager: ResMut<InputManager>,
) {
    let is_f2_pressed = input_manager.is_key_pressed(&VirtualKeyCode::F2);
    if is_f2_pressed {
        let wireframe_only = !game_renderer.is_wireframe_only();
        game_renderer.set_display_wireframe_only(&renderer.render_context, wireframe_only);
    };
}
