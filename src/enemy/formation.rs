use crate::{WinSize, BASE_SPEED, FORMATION_MEMBERS_MAX};
use bevy::prelude::{Component, Resource};
use rand::{thread_rng, Rng};

/// Component - Enemy Formation (per enemy)
#[derive(Clone, Component)]
pub struct Formation {
	pub start: (f32, f32),
	pub radius: (f32, f32),
	pub pivot: (f32, f32),
	pub speed: f32,
	pub angle: f32, // change per tick
}

/// Resource - Formation Maker
#[derive(Default, Resource)]
pub struct FormationMaker {
	current_template: Option<Formation>,
	current_members: u32,
}

/// Formation factory implementation
impl FormationMaker {
	pub fn make(&mut self, win_size: &WinSize) -> Formation {}
}
