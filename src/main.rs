use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use components::{Movable, Velocity}; // Import Velocity along with Movable
use player::PlayerPlugin;

mod player;
mod components;

// region:  ---Asset Constants
const PLAYER_SPRITE: &str = "player_a_01.png"; // Path to player sprite
const PLAYER_SIZE: (f32, f32) = (144., 75.); // Player sprite size
const SPRITE_SCALE: f32 = 0.5;
const BASE_SPEED: f32 = 500.0; // Example player base speed
const TIME_STEP: f32 = 1.0 / 60.0; // Assuming 60 FPS
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32)= (9., 54.);
// endregion: ---Asset Constants

//region: ---Resources
#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<Image>,
    pub player_laser: Handle<Image>, // Make the field public
}
//endregion: ---Resources

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
        .add_startup_system(setup_system)
        .add_system(movable_system)
        .add_plugin(PlayerPlugin)
        .run();
}

// System to set up the game scene (camera, player sprite)
fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>, // Query for the primary window
) {
    // Spawn the 2D camera
    commands.spawn(Camera2dBundle::default());

    // Get a reference to the primary window
    if let Ok(window) = window_query.get_single() {
        let (win_w, win_h) = (window.width(), window.height());

        // Add WinSize resource
        let win_size = WinSize { w: win_w, h: win_h };
        commands.insert_resource(win_size);

        // Add GameTextures resource
        let game_textures = GameTextures {
            player: asset_server.load(PLAYER_SPRITE),
            player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        };
        commands.insert_resource(game_textures);
    } else {
        eprintln!("Error: Could not get primary window.");
    }
}

// System to move entities marked as `Movable`
fn movable_system(
    mut query: Query<(&Velocity, &mut Transform, &Movable)>,
) {
    for (velocity, mut transform, _movable) in query.iter_mut() {
        transform.translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        transform.translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}
