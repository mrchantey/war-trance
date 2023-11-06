use bevy::prelude::*;


#[derive(Debug, Component)]
pub struct DespawnMarker;

pub fn despawn_marked<T: Component>(world: &mut World) {
	world
		.query_filtered::<Entity, With<T>>()
		.iter(world)
		//copy entities to new vec to allow mutable world access
		.collect::<Vec<_>>()
		.iter()
		.for_each(|entity| {
			despawn_recursive(world, *entity);
			// world.despawn(*entity);
			// world.get_mut::<Children>(enti
		});
}


fn despawn_recursive(world: &mut World, entity: Entity) {
	if let Some(children) = world.get::<Children>(entity) {
		for e in children
			.into_iter()
			.map(|e| *e)
			.collect::<Vec<_>>()
			.into_iter()
		{
			despawn_recursive(world, e);
		}
	}
	if !world.despawn(entity) {
		panic!("entities retrieved by world should always exist");
	}
}
