use bevy::prelude::*;
use game::GamePlugin;

pub mod game;
pub mod alien;
pub mod resolution;
pub mod player;

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Astro Intruders".to_string(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(800.0, 800.0).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                GamePlugin
            )
        )
        .run();
}