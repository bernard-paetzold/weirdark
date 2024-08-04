pub use dispatcher::UnifiedDispatcher;

mod dispatcher;

mod visibility_system;
mod lighting_system;
mod map_index_system;
mod power_system;
mod interaction_system;

pub fn build() -> Box<dyn UnifiedDispatcher + 'static> {
    dispatcher::new()
}