pub use lighting_system::LightingSystem;
use map_index_system::MapIndexSystem;
use power_system::PowerSystem;
use specs::prelude::World;
use visibility_system::VisibilitySystem;
use interaction_system::InteractionSystem;

use super::*;

#[cfg(target_arch = "wasm32")]
#[macro_use]
mod single_thread;

#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
mod multi_thread;

#[cfg(target_arch = "wasm32")]
pub use single_thread::*;

#[cfg(not(target_arch = "wasm32"))]
pub use multi_thread::*;

pub trait UnifiedDispatcher {
    fn run_now(&mut self, ecs : *mut World);
}

construct_dispatcher!(
    (MapIndexSystem, "map_index", &[]),
    (PowerSystem, "power", &[]),
    (InteractionSystem, "interaction", &[]),
    (VisibilitySystem, "visibility", &[]),
    (LightingSystem, "lighting", &[])
);

pub fn new() -> Box<dyn UnifiedDispatcher + 'static> {
    new_dispatch()
}
