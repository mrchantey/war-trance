use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;



#[derive(Component,Deref,DerefMut)]
pub struct DamageOnCollide(pub f32);

pub fn damage_on_collide(
	mut commands: Commands,
	mut collision_events: EventReader<CollisionEvent>,
	query: Query<(Entity, &TeamId, &DamageOnCollide)>,
	mut enemies: Query<(Entity, &TeamId, &mut Health)>,
) {	

	let mut for_each_collision = |a: Entity, b: Entity| {
		if let Ok((entity,team_id,damage)) = query.get(a) 
		&& let Ok((enemy,enemy_id,mut health)) = enemies.get_mut(b) 
			&& *team_id != *enemy_id {
				commands.entity(entity).insert(DespawnMarker);

				**health -= **damage;
				if **health <= 0.0 {
					commands.entity(enemy).insert(DespawnMarker);
			}
		}
	};


	for collision_event in collision_events.iter() {
		match collision_event {
			CollisionEvent::Started(a, b, _) => {
				for_each_collision(*a,*b);
				for_each_collision(*b,*a);
			}
			CollisionEvent::Stopped(_, _, _) => {}
		}
	}
}
