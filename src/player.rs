use crate::{components::{Movable, Player, Velocity}, GameTextures, WinSize, BASE_SPEED, PLAYER_SIZE, SPRITE_SCALE, TIME_STEP};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(player_spawn_system.in_base_set(StartupSet::PostStartup)) // Correct API for Bevy 0.10+
            .add_system(player_keyboard_event_system)
            .add_system(player_movement_system) // Added the missing system
            .add_system(player_fire_system);
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
    .insert(Movable { auto_despawn: false }) // Ensure Movable is correctly defined
    .insert(Velocity { x: 0., y: 0. });
}

fn player_fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_tf) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);
            let x_offset = PLAYER_SIZE.0 / 4. * SPRITE_SCALE -5.;
            
            let mut spawn_laser = |x_offset: f32|{
                commands.spawn(SpriteBundle {
                    texture: game_textures.player_laser.clone(),
                    transform: Transform {
                        translation: Vec3::new(x + x_offset, y + 15. , 0.),
                        scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Movable { auto_despawn: true })
                .insert(Velocity { x: 0., y: 1. }); 
            };

            spawn_laser(x_offset);
            spawn_laser(-x_offset);
        }
    }
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if kb.pressed(KeyCode::Left) {
            -1.
        } else if kb.pressed(KeyCode::Right) {
            1.
        } else {
            0.
        };
    }
}

fn player_movement_system(
    mut query: Query<(&Velocity, &mut Transform), With<Player>>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        transform.translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}
