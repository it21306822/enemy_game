use crate::{components::{Player, Velocity}, GameTextures, WinSize, PLAYER_SIZE, SPRITE_SCALE};
use bevy::prelude::*; // Ensure you're importing everything needed

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(player_spawn_system.in_base_set(StartupSet::PostStartup)) // Correct API for Bevy 0.10+
            .add_system(player_movement_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let bottom = -win_size.h / 2.0;

    // Spawn the player sprite using the texture
    commands.spawn(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform {
            translation: Vec3::new(
                0.0,
                bottom + PLAYER_SIZE.1 / 2.0 * SPRITE_SCALE + 5.0,
                10.0,
            ),
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
        ..Default::default()
    })
    .insert(Player)
    .insert(Velocity { x: 1., y: 0. });
}

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}
