use crate::Health;
use crate::TeamId;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Debug, Default, Resource)]
pub struct UnitCount {
	teams: HashMap<TeamId, usize>,
	total: usize,
}

impl UnitCount {
	pub fn new() -> Self {
		Self {
			teams: HashMap::new(),
			total: 0,
		}
	}

	pub fn get(&self, team: TeamId) -> usize {
		match self.teams.get(&team) {
			Some(count) => *count,
			None => 0,
		}
	}

	pub fn total(&self) -> usize { self.total }
	pub fn is_victory(&self, team: TeamId) -> bool {
		//TODO alliances
		self.get(team) == self.total
	}

	pub fn fraction(&self, team: TeamId) -> f32 {
		if self.total == 0 {
			return 0.;
		} else {
			self.get(team) as f32 / self.total as f32
		}
	}

	pub fn recalculate(&mut self, query: Query<&TeamId, With<Health>>) {
		self.teams.clear();
		self.total = 0;
		for team in query.iter() {
			self.add(*team);
		}
	}


	fn add(&mut self, team: TeamId) {
		self.total += 1;
		match self.teams.get_mut(&team) {
			Some(count) => *count += 1,
			None => {
				self.teams.insert(team, 1);
			}
		}
	}
}


pub fn update_team_count(
	query: Query<&TeamId, With<Health>>,
	mut team_count: ResMut<UnitCount>,
) {
	team_count.recalculate(query);
}
