use bevy::prelude::*;

use crate::resolution::Resolution;

pub struct LaserProjectilePlugin;

#[derive(Component)]
pub struct LaserProjectile {
    pub speed: f32,
}

fn update_laser_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &LaserProjectile, &mut Transform)>,
    time: Res<Time>,
    resolution: Res<Resolution>
) {
    for(entity, projectile, mut transform) in projectile_query.iter_mut() {
        transform.translation.y += projectile.speed * time.delta_secs();

        // Despawn the laser projectile once off-screen
        if transform.translation.y > resolution.screen_dimensions.y * 0.5 {
            commands.entity(entity).despawn();
        }
    }
}

fn update_alien_interactions() {

}

impl Plugin for LaserProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_laser_projectiles,
            update_alien_interactions,
        ));
    }
}