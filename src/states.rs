use rltk::VirtualKeyCode;

use crate::{gui, vectors::Vector3i};

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum RunState { 
    AwaitingInput { turn_time:  f32 }, 
    PreRun,
    PlayerTurn { turn_time:  f32 },
    NPCTurn, 
    MainMenu { menu_selection: gui::MainMenuSelection },
    SaveGame,
    InteractGUI { range: usize, source: Vector3i, target: Vector3i, prev_mouse_position: Vector3i },
    HandleOtherInput { next_runstate: std::sync::Arc<RunState>, key: VirtualKeyCode }
}