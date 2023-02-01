use self::formation::{Formation, FormationMaker};
use crate::components::{Enemy, FromEnemy, Laser, Movable, SpriteSize, Velocity};
use crate::{
	EnemyCount, GameTextures, WinSize, ENEMY_LASER_SIZE, ENEMY_MAX, ENEMY_SIZE, SPRITE_SCALE,
	TIME_STEP,
};
use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;
use bevy::time::FixedTimestep;
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

mod formation;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
	fn build(&self, app: &mut App) {
}

fn enemy_spawn_system(
	mut commands: Commands,
	game_textures: Res<GameTextures>,
	mut enemy_count: ResMut<EnemyCount>,
	mut formation_maker: ResMut<FormationMaker>,
	win_size: Res<WinSize>,
) {
}

fn enemy_fire_criteria() -> ShouldRun {
	if thread_rng().gen_bool(1. / 60.) {
		ShouldRun::Yes
	} else {
		ShouldRun::No
	}
}

fn enemy_fire_system(
	mut commands: Commands,
	game_textures: Res<GameTextures>,
	enemy_query: Query<&Transform, With<Enemy>>,
) {
}

fn enemy_movement_system(mut query: Query<(&mut Transform, &mut Formation), With<Enemy>>) {}
