use event_system::EventSystem;
pub use lighting_system::LightingSystem;
//use map_index_system::MapIndexSystem;
use power_system::PowerSystem;
use specs::prelude::World;
use state_align_system::StateAlignSystem;
use visibility_system::VisibilitySystem;
use atmosphere_system::AtmosphereSystem;
use biology_system::BiologySystem;

use super::*;

#[macro_use]
mod multi_thread;

pub use multi_thread::*;

pub trait UnifiedDispatcher {
    fn run_now(&mut self, ecs : *mut World);
}


construct_dispatcher!(
    //(MapIndexSystem, "map_index", &[]),
    (AtmosphereSystem, "atmosphere", &[]),
    (BiologySystem, "biology", &[]),
    (EventSystem, "events", &[]),
    (StateAlignSystem, "state_align", &[]),
    (PowerSystem, "power", &[]),
    (VisibilitySystem, "visibility", &[]),
    (LightingSystem, "lighting", &[])
);

pub fn new() -> Box<dyn UnifiedDispatcher + 'static> {
    new_dispatch()
}
