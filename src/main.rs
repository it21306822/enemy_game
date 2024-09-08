use bevy::prelude::*;
use bevy::window::WindowResolution;

const PLAYER_SPRITE: &str = "player_a_01.png"; // Path to player sprite
const PLAYER_SIZE: (f32, f32) = (144., 75.); // Player sprite size

fn main() {
    App::new()
        // Set the background color to dark
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        // Load default plugins and configure the window
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Invaders!".to_string(), // Window title
                resolution: WindowResolution::new(598.0, 676.0), // Window size
                ..Default::default()
            }),
            ..Default::default()
        }))
        // Add the setup system to initialize the game
        .add_systems(Startup, setup_system) // Use `add_systems` in Bevy 0.10+
        .run();
}

// System to set up the game scene (camera, player sprite)
fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the 2D camera
    commands.spawn(Camera2dBundle::default());

    // Spawn the player sprite using the texture
    commands.spawn(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE), // Load player texture
        sprite: Sprite {
            custom_size: Some(Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1)), // Set player sprite size
            ..Default::default()
        },
        ..Default::default()
    });
}
