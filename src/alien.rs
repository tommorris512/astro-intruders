use std::f32;

use bevy::prelude::*;

use crate::resolution::Resolution;

pub struct AlienPlugin;

#[derive(Component)]
pub struct Alien;

#[derive(Resource)]
pub struct AlienManager {
    pub direction: f32,
    pub distance_from_boundary: f32,
    pub downshift_alines: bool,
}

// Alien group constants
const WIDTH: i32 = 10;
const HEIGHT: i32 = 5;
const SPACING: f32 = 24.0;
const SPEED: f32 = 100.0;
const ALIEN_DOWNSHIFT_DISTANCE: f32 = 32.0;

fn setup_aliens(mut commands: Commands, asset_server: Res<AssetServer>, resolution: Res<Resolution>) {
    commands.insert_resource(AlienManager {
        direction: 1.0,
        distance_from_boundary: 0.0,
        downshift_alines: false,
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
    resolution: Res<Resolution>,
    time: Res<Time>,
) {
    for (_alien, mut transform) in alien_query.iter_mut() {
        transform.translation.x += time.delta_secs() * alien_manager.direction * SPEED;

        if transform.translation.x.abs() > resolution.screen_dimensions.x * 0.5 {
            alien_manager.downshift_alines = true;
            alien_manager.distance_from_boundary = resolution.screen_dimensions.x * alien_manager.direction * 0.5 - transform.translation.x;
        }
    }
}

fn handle_alien_downshift(
    mut alien_query: Query<(&mut Alien, &mut Transform)>,
    mut alien_manager: ResMut<AlienManager>,
) {
    if alien_manager.downshift_alines {
        alien_manager.downshift_alines = false;
        alien_manager.direction *= -1.0;

        for(_alien, mut transform) in alien_query.iter_mut() {
            transform.translation.x += alien_manager.distance_from_boundary;
            transform.translation.y -= ALIEN_DOWNSHIFT_DISTANCE;
        }
    }
}

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_aliens)
            .add_systems(Update, 
                (
                    update_aliens, 
                    handle_alien_downshift
                )
            );
    }
}
