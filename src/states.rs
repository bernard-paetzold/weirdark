use crate::{gui, vectors::Vector3i};

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { 
    AwaitingInput, 
    PreRun,
    PlayerTurn,
    NPCTurn, 
    MainMenu { menu_selection: gui::MainMenuSelection },
    SaveGame,
    InteractGUI { range: usize, source: Vector3i, target: Vector3i },
}