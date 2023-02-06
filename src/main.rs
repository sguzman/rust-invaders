#![allow(unused)] // silence unused warnings while exploring (to comment out)

use bevy::prelude::*;
use bevy::render::mesh::shape::Circle;
use bevy::sprite::collide_aabb::collide;
use bevy::{math::Vec3Swizzles, sprite::MaterialMesh2dBundle};
use bevy_editor_pls::*;
use std::collections::HashSet;

// Import meshes and materials

const SPRITE_SCALE: f32 = 0.5;

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

#[derive(Component)]
pub struct Velocity {
	pub x: f32,
	pub y: f32,
}

#[derive(Component)]
pub struct CircleEntity;

// endregion: --- Game Constants

// region:    --- Resources
#[derive(Resource)]
pub struct WinSize {
	pub w: f32,
	pub h: f32,
}

fn main() {
	App::new()
		.insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			window: WindowDescriptor {
				title: "Rust Invaders!".to_string(),
				width: 598.0,
				height: 676.0,
				..Default::default()
			},
			..Default::default()
		}))
		.add_plugin(EditorPlugin)
		.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
		.add_plugin(bevy::diagnostic::EntityCountDiagnosticsPlugin)
		//.add_plugin(CommonPlugin)
		.add_startup_system(setup_system)
		.add_system(move_circle_system)
		.add_system(circle_keyboard_event_system)
		.run();
}

fn setup_system(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut windows: ResMut<Windows>,
) {
	// camera
	commands.spawn(Camera2dBundle::default());

	// capture window size
	let window = windows.get_primary_mut().unwrap();
	let (win_w, win_h) = (window.width(), window.height());

	// position window (for tutorial)
	// window.set_position(IVec2::new(2780, 4900));

	// add WinSize resource
	let win_size = WinSize { w: win_w, h: win_h };
	commands.insert_resource(win_size);

	commands
		.spawn(MaterialMesh2dBundle {
			mesh: meshes.add(shape::Circle::new(50.).into()).into(),
			material: materials.add(ColorMaterial::from(Color::RED)),
			transform: Transform::from_translation(Vec3::new(-10., 0., 0.)),
			..default()
		})
		.insert(Velocity { x: 0., y: 0. })
		.insert(CircleEntity);
}

// Movable system for circle	entities
fn move_circle_system(
	mut commands: Commands,
	win_size: Res<WinSize>,
	mut query: Query<(Entity, &Velocity, &mut Transform, &CircleEntity)>,
) {
	for (entity, velocity, mut transform, circle) in query.iter_mut() {
		let translation = &mut transform.translation;
		translation.x += velocity.x * TIME_STEP * BASE_SPEED;
		translation.y += velocity.y * TIME_STEP * BASE_SPEED;

		// despawn when out of screen
		const MARGIN: f32 = 200.;
		if translation.y > win_size.h / 2. + MARGIN
			|| translation.y < -win_size.h / 2. - MARGIN
			|| translation.x > win_size.w / 2. + MARGIN
			|| translation.x < -win_size.w / 2. - MARGIN
		{
			commands.entity(entity).despawn();
		}
	}
}

// Function system that queries circles and moves them
fn circle_keyboard_event_system(
	kb: Res<Input<KeyCode>>,
	// Query circle entities with a Velocity component
	mut query: Query<(&mut Velocity, &CircleEntity)>,
) {
	// Move Circle entity by velocity on WASD keys press
	for (mut velocity, circle) in query.iter_mut() {
		if kb.pressed(KeyCode::W) {
			velocity.y = 1.;
		} else if kb.pressed(KeyCode::S) {
			velocity.y = -1.;
		} else {
			velocity.y = 0.;
		}

		if kb.pressed(KeyCode::A) {
			velocity.x = -1.;
		} else if kb.pressed(KeyCode::D) {
			velocity.x = 1.;
		} else {
			velocity.x = 0.;
		}
	}
}
