pub use lighting_system::LightingSystem;
use map_index_system::MapIndexSystem;
use power_system::PowerSystem;
use specs::prelude::World;
use state_align_system::StateAlignSystem;
use visibility_system::VisibilitySystem;
use interaction_system::InteractionSystem;

use super::*;

#[macro_use]
mod multi_thread;

pub use multi_thread::*;

pub trait UnifiedDispatcher {
    fn run_now(&mut self, ecs : *mut World);
}

construct_dispatcher!(
    (MapIndexSystem, "map_index", &[]),
    (InteractionSystem, "interaction", &[]),
    (StateAlignSystem, "state_align", &[]),
    (PowerSystem, "power", &[]),
    (VisibilitySystem, "visibility", &[]),
    (LightingSystem, "lighting", &[])
);

pub fn new() -> Box<dyn UnifiedDispatcher + 'static> {
    new_dispatch()
}
