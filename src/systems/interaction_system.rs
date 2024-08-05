
use specs::prelude::*;

use crate::{gamelog::GameLog, InteractIntent, Interactable, Name, PowerSwitch};

pub struct InteractionSystem {}

impl<'a> System<'a> for InteractionSystem {
    type SystemData = ( Entities<'a>,
                        WriteExpect<'a, GameLog>,
                        WriteStorage<'a, InteractIntent>,
                        WriteStorage<'a, PowerSwitch>,
                        ReadStorage<'a, Name>,
                      );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut game_log,
            mut interact_intents,
            mut power_switches,
            names,
        ) = data;


        let mut cleared_intents = Vec::new();

        //TODO: Add other interactions
        for (entity, interact_intent) in (&entities, &interact_intents).join() {
            if let Some(power_switch) = power_switches.get_mut(interact_intent.target) {
                if power_switch.interaction_id == interact_intent.interaction_id {
                    power_switch.interact();

                    if let Some(name) = names.get(entity) {
                        game_log.entries.push(format!("{}: {}", name.name, interact_intent.interaction_description.clone()));
                    }
                    else {
                        game_log.entries.push("Invalid intent".to_string());
                    }
                    cleared_intents.push(entity);
                }
            }
        }

        //Clear intents
        for entity in cleared_intents.iter() {
            interact_intents.remove(*entity);
        }   
    }
}

pub fn get_entity_interactions(ecs: &World, entity: Entity) -> Vec<(String, String, u32)> {
    let power_switches = ecs.read_storage::<PowerSwitch>();

    let mut interactables = Vec::new();

    //TODO: Add any other interactable components
    if let Some(power_switch) = power_switches.get(entity) {
        interactables.push((
            power_switch.interaction_id.clone(),
            power_switch.interaction_description.clone(),
            entity.id()
        ));
    }

    interactables
}
