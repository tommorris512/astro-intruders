use bevy::prelude::*;

use crate::{alien, laser_projectile, player, resolution};

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
                    laser_projectile::LaserProjectilePlugin,
                    alien::AlienPlugin,
                )
            )
            .add_systems(Startup, setup_scene);
    }
}