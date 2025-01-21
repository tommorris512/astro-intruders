use bevy::prelude::*;

use crate::resolution::Resolution;
pub struct PlayerPlugin;

#[derive(Component)]
struct Player {
    pub shoot_timer: f32,
}

// Define default player constants
const PLAYER_SPEED: f32 = 300.0;
const LASER_SPEED: f32 = 300.0;
const LASER_FIRE_COOLDOWN: f32 = 0.3;

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<Resolution>
) {
    let player_texture: Handle<Image> = asset_server.load("ship.png");

    let y_offset: f32 = 50.0 - resolution.screen_dimensions.y * 0.5;

    commands.spawn((
        Player {
            shoot_timer: 0.0
        },
        Sprite::from_image(
            player_texture
        ),
        Transform {
            translation: Vec3::new(
                0.0,
                y_offset,
                0.0
            ),
            scale: Vec3::splat(resolution.pixel_draw_ratio),
            ..Default::default()
        },
    ));
}

fn update_player(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    resolution: Res<Resolution>
) {
    // Obtain a single, mutable instance for the player
    let(mut player, mut transform) = player_query.single_mut();

    let mut horizontal: f32 = 0.0;

    if keys.pressed(KeyCode::ArrowLeft) {
        horizontal -= 1.0;
    }

    if keys.pressed(KeyCode::ArrowRight) {
        horizontal += 1.0;
    }

    transform.translation.x += horizontal * time.delta_secs() * PLAYER_SPEED;

    let left_bound: f32 = - resolution.screen_dimensions.x * 0.5;
    let right_bound: f32 = resolution.screen_dimensions.x * 0.5;

    if transform.translation.x < left_bound {
        transform.translation.x = left_bound
    }

    if transform.translation.x > right_bound {
        transform.translation.x = right_bound
    }

    player.shoot_timer -= time.delta_secs();

    if keys.pressed(KeyCode::Space) {

    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_player)
            .add_systems(Update, update_player);
    }
}