use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Helper methods on [`bevy_rapier2d::CollisionEvent`]
///
/// Found with the plugin for Bevy Retrograde
pub trait CollisionEventExt {
    fn entities(&self) -> (Entity, Entity);
    fn is_started(&self) -> bool;
    fn is_stopped(&self) -> bool;
}

impl CollisionEventExt for CollisionEvent {
    /// Get the entities involved in the collision
    fn entities(&self) -> (Entity, Entity) {
        match self {
            CollisionEvent::Started(ent1, ent2, _) | CollisionEvent::Stopped(ent1, ent2, _) => {
                (*ent1, *ent2)
            }
        }
    }

    /// Whether or not the contact has just started
    fn is_started(&self) -> bool {
        match self {
            CollisionEvent::Started(_, _, _) => true,
            CollisionEvent::Stopped(_, _, _) => false,
        }
    }

    /// Whether or not the contact has just stopped
    fn is_stopped(&self) -> bool {
        !self.is_started()
    }
}
