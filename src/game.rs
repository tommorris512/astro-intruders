use bevy::prelude::*;

use crate::{alien, resolution};

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
                    alien::AlienPlugin, 
                    resolution::ResolutionPlugin
                )
            )
            .add_systems(Startup, setup_scene);
    }
}