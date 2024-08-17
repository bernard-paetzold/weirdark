pub use dispatcher::UnifiedDispatcher;

mod dispatcher;

mod visibility_system;
mod lighting_system;
mod map_index_system;
pub mod power_system;
pub mod event_system;
mod state_align_system;
mod atmosphere_system;
mod biology_system;

pub fn build() -> Box<dyn UnifiedDispatcher + 'static> {
    dispatcher::new()
}