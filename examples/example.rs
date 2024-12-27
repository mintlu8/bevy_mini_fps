use bevy::{prelude::*, window::PresentMode};
use bevy_mini_fps::fps_plugin;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    title: "Bevy Mini FPS".into(),
                    ..Default::default()
                }),
                ..Default::default()
            }))
        .add_plugins(fps_plugin!())
        .add_plugins(|app: &mut App| {
            app.world_mut().spawn((
                Camera2d,
                Camera {
                    clear_color: ClearColorConfig::Custom(Color::srgb(0., 0.5, 0.2)),
                    ..Default::default()
                }
            ));
        })
        .run();
}