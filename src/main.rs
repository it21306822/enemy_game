mod player;

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use player::PlayerPlugin;

// region:  ---Asset Constants
const PLAYER_SPRITE: &str = "player_a_01.png"; // Path to player sprite
const PLAYER_SIZE: (f32, f32) = (144., 75.); // Player sprite size
const SPRITE_SCALE: f32 = 0.5;
// endregion: ---Asset Constants

//region: ---Resources
#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<Image>, // Make the field public
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

        .add_plugin(PlayerPlugin)
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

        // Add WinSize resource
        let win_size = WinSize { w: win_w, h: win_h };
        commands.insert_resource(win_size);

        // Add GameTextures resource
        let game_textures = GameTextures {
            player: asset_server.load(PLAYER_SPRITE),
        };
        commands.insert_resource(game_textures);

        // Calculate player position (bottom of the screen)
        let bottom = -win_h / 2.0;

        // Spawn the player sprite using the texture
        commands.spawn(SpriteBundle {
            texture: asset_server.load(PLAYER_SPRITE), // Load player texture
            transform: Transform {
                translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.), // Position player
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
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
