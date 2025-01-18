use bevy::prelude::*;

pub struct ResolutionPlugin;

#[derive(Resource)]
pub struct Resolution {
    pub screen_dimensions: Vec2,

    // Ratio to draw sprites at
    pub pixel_draw_ratio: f32,
}

fn setup_resolution(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.single();

    commands.insert_resource(Resolution {
        screen_dimensions: Vec2::new(window.width(), window.height()),
        pixel_draw_ratio: 2.0,
    });
}

impl Plugin for ResolutionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_resolution);
    }
}