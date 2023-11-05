use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Bundle)]
pub struct ColliderBundle {
	pub collider: Collider,
	pub rigidbody: RigidBody,
	pub active_events: ActiveEvents,
	pub active_collision_types: ActiveCollisionTypes,
	// pub ccd: Ccd,
}

impl ColliderBundle {
	pub fn new(collider: Collider) -> Self {
		Self {
			collider,
			rigidbody: RigidBody::KinematicPositionBased,
			active_events: ActiveEvents::COLLISION_EVENTS,
			active_collision_types: ActiveCollisionTypes::default()
				| ActiveCollisionTypes::KINEMATIC_KINEMATIC,
			// ccd: Ccd::enabled(),
		}
	}
}


pub fn display_events(mut collision_events: EventReader<CollisionEvent>) {
	for collision_event in collision_events.iter() {
		match collision_event {
			CollisionEvent::Started(_, _, _) => {
				println!("Received collision event: {:?}", collision_event);
			}
			CollisionEvent::Stopped(_, _, _) => {}
		}
	}
}
