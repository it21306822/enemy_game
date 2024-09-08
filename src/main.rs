use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution}; // Import PrimaryWindow and WindowResolution

const PLAYER_SPRITE: &str = "player_a_01.png"; // Path to player sprite
const PLAYER_SIZE: (f32, f32) = (144., 75.); // Player sprite size

fn main() {
    App::new()
        // Set the background color to dark
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        // Configure the window
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust Invaders!".to_string(), // Window title
                resolution: WindowResolution::new(598.0, 676.0), // Window size
                ..Default::default()
            }),
            ..Default::default()
        }))
        // Add the setup system to initialize the game
        .add_startup_system(setup_system) // Use `add_startup_system` for Bevy 0.10+
        .run();
}

// System to set up the game scene (camera, player sprite)
fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>, // Query for the primary window
) {
    // Spawn the 2D camera
    commands.spawn(Camera2dBundle::default());

    // Get a mutable reference to the primary window
    if let Ok(mut window) = window_query.get_single_mut() {
        let (win_w, win_h) = (window.width(), window.height());

        // Comment out the set_position since it's no longer supported directly
        // window.set_position(IVec2::new(2780, 4900)); 

        // Calculate player position (bottom of the screen)
        let bottom = -win_h / 2.;

        // Spawn the player sprite using the texture
        commands.spawn(SpriteBundle {
            texture: asset_server.load(PLAYER_SPRITE), // Load player texture
            transform: Transform {
                translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. + 5., 10.), // Position player
                ..Default::default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1)), // Set player sprite size
                ..Default::default()
            },
            ..Default::default()
        });
    } else {
        eprintln!("Error: Could not get primary window.");
    }
}
