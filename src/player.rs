use crate::{GameTextures, WinSize, PLAYER_SIZE,SPRITE_SCALE};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
       // app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>, // Use win_size as a reference, preventing move
) {
    let bottom = -win_size.h / 2.0;

    // Spawn the player sprite using the texture
    commands.spawn(SpriteBundle {
        texture: game_textures.player.clone(), // Use loaded texture
        transform: Transform {
            translation: Vec3::new(
                0.0,
                bottom + PLAYER_SIZE.1 / 2.0 * SPRITE_SCALE + 5.0,
                10.0,
            ), // Position player
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
            ..Default::default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(
                PLAYER_SIZE.0 * SPRITE_SCALE,
                PLAYER_SIZE.1 * SPRITE_SCALE,
            )),
            ..Default::default()
        },
        global_transform: Default::default(), // Set a default global transform
        visibility: Visibility::default(),    // Set default visibility
        computed_visibility: ComputedVisibility::default(), // Set default computed visibility
    });
}