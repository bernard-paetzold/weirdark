use rltk::VirtualKeyCode;
use specs::Entity;

use crate::{gui, vectors::Vector3i};

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    Ticking,
    MainMenu {
        menu_selection: gui::MainMenuSelection,
    },
    SaveGame,
    InteractGUI {
        range: usize,
        source: Vector3i,
        target: Vector3i,
        prev_mouse_position: Vector3i,
        selected_entity: Option<Entity>,
    },
    HandleOtherInput {
        next_runstate: std::sync::Arc<RunState>,
        key: VirtualKeyCode,
    },
    Simulation {
        steps: usize,
    },
    ShowInventory {
        id: u32,
        selected_item: Option<Entity>,
    },
}
