use bevy::prelude::*;
use bevy::window::WindowResolution;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Invaders!".to_string(),
                resolution: WindowResolution::new(598.0, 676.0),
                ..Default::default()
            }),
            ..Default::default()
        }))
        
        .run();
}

fn setup_system(mut commands: Commands) {
    // Add setup logic here, if needed
}
