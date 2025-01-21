use bevy::prelude::*;

use crate::{player, alien, resolution};

pub struct GamePlugin;

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d{
        ..default()
    });
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                (
                    resolution::ResolutionPlugin,
                    player::PlayerPlugin,
                    alien::AlienPlugin, 
                )
            )
            .add_systems(Startup, setup_scene);
    }
}