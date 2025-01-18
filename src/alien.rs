use std::f32;

use bevy::prelude::*;

use crate::resolution::Resolution;

pub struct AlienPlugin;

#[derive(Component)]
pub struct Alien;

#[derive(Resource)]
pub struct AlienManager {
    pub direction: f32,
}

// Alien group constants
const WIDTH: i32 = 10;
const HEIGHT: i32 = 5;
const SPACING: f32 = 24.0;
const SPEED: f32 = 100.0;

fn setup_aliens(mut commands: Commands, asset_server: Res<AssetServer>, resolution: Res<Resolution>) {
    commands.insert_resource(AlienManager {
        direction: 1.0,
    });
    
    let alien_texture = asset_server.load("alien.png");

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let total_width: f32 = (WIDTH as f32 - 1.0) * SPACING;
            let total_height: f32 = (HEIGHT as f32 - 1.0) * SPACING;

            // Offset to center the group of aliens
            let x_offset: f32 = - total_width * 0.5;
            let y_offset: f32 = resolution.screen_dimensions.y * 0.5 - total_height - SPACING;

            let position: Vec3 = Vec3::new(
                x as f32 * SPACING + x_offset, 
                y as f32 * SPACING + y_offset,
                0.0,
            );

            commands.spawn((
                Alien,
                Sprite::from_image(
                    alien_texture.clone()
                ),
                Transform {
                    translation: position,
                    scale: Vec3::splat(resolution.pixel_draw_ratio),
                    ..Default::default()
                },
            ));
        }
    }    
}

fn update_aliens(
    mut alien_query: Query<(&Alien, &mut Transform)>, 
    mut alien_manager: ResMut<AlienManager>, 
    time: Res<Time>,
) {
    for (alien, mut transform) in alien_query.iter_mut() {
        transform.translation.x += time.delta_secs() * alien_manager.direction * SPEED;
    }
}

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_aliens)
            .add_systems(Update, update_aliens);
    }
}
