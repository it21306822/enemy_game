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
        .add_systems(Startup, setup_system)
        .run();
}


fn setup_system(mut commands: Commands) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // add rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(150., 150.)),
            ..Default::default()
        },
        ..Default::default()
    });
}