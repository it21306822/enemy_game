use bevy::prelude::*;
use crate::{GameTextures, SPRITE_SCALE};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // Use system sets to define order instead of string labels
        app.add_startup_system(enemy_spawn_system.in_base_set(StartupSet::PostStartup)); // Post setup for spawning enemies
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
) {
    commands.spawn(SpriteBundle {
        texture: game_textures.enemy.clone(),
        
        ..Default::default()
    });
}
