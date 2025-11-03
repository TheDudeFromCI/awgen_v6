//! This module implements the ChildList component for dynamic child list
//! bundles.

use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;

/// A component that holds a list of child spawner commands.
#[derive(Default, Component)]
#[component(storage = "SparseSet")]
pub struct ChildList {
    /// The list of child spawner commands.
    #[allow(clippy::type_complexity)]
    children: Vec<Box<dyn FnOnce(&mut RelatedSpawnerCommands<ChildOf>) + Send + Sync>>,
}

impl ChildList {
    /// Creates a [`ChildList`] using the provided spawner function.
    pub fn from(spawner: impl FnOnce(&mut ChildList)) -> Self {
        let mut child_list = ChildList::default();
        spawner(&mut child_list);
        child_list
    }

    /// Adds a child spawner command to the list.
    pub fn add_child(&mut self, bundle: impl Bundle) {
        self.children.push(Box::new(
            move |spawner: &mut RelatedSpawnerCommands<ChildOf>| {
                spawner.spawn(bundle);
            },
        ));
    }

    /// Spawns all children using the provided spawner commands.
    fn spawn_children(&mut self, spawner: &mut RelatedSpawnerCommands<ChildOf>) {
        for child_spawner in self.children.drain(..) {
            child_spawner(spawner);
        }
    }
}

/// System that processes entities with the p`ChildList`[ component upon
/// insertion, spawning their children and removing the component.
pub(super) fn on_spawn(
    trigger: On<Insert, ChildList>,
    mut query: Query<&mut ChildList>,
    mut commands: Commands,
) {
    let entity = trigger.entity;
    let mut child_list = query.get_mut(entity).unwrap();

    commands
        .entity(entity)
        .remove::<ChildList>()
        .with_children(|spawner| {
            child_list.spawn_children(spawner);
        });
}
