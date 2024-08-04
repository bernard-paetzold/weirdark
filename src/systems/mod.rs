pub use dispatcher::UnifiedDispatcher;

mod dispatcher;

mod visibility_system;
mod lighting_system;
mod map_index_system;

pub fn build() -> Box<dyn UnifiedDispatcher + 'static> {
    dispatcher::new()
}