use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use crate::{
    core::physics::{PhysicsSet, Position},
    entities::player::Player,
};

#[derive(Resource, Reflect)]
struct MapLimit {
    // TODO: Reconsider using existing Radius component
    radius: f32,
    /// Precomputed for performance optimizations
    radius_squared: f32,
}

#[derive(Event)]
pub struct DeathEvent(pub Entity);

pub struct MapLimitPlugin;
impl Plugin for MapLimitPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MapLimit>()
            .add_event::<DeathEvent>()
            .insert_resource(MapLimit {
                radius: 3000.0,
                radius_squared: 3000.0f32.powi(2),
            })
            .add_systems(GgrsSchedule, check_outsiders.after(PhysicsSet::Movement));
    }
}

fn check_outsiders(
    mut commands: Commands,
    mut death_events: EventWriter<DeathEvent>,
    query: Query<(Entity, &Position, Has<Player>)>,
    limit: Res<MapLimit>,
) {
    for (entity, position, is_player) in query.iter() {
        if position.length_squared() > limit.radius_squared {
            if is_player {
                death_events.send(DeathEvent(entity));
            } else {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
