use specs::prelude::*;

use crate::{entities::intents::InteractIntent, gamelog::GameLog,  entities::intents::Interactable, Door, Name, PowerNode, PowerSwitch};

pub struct EventSystem {}

impl<'a> System<'a> for EventSystem {
    type SystemData = ( Entities<'a>,
                        WriteExpect<'a, GameLog>,
                        WriteStorage<'a, InteractIntent>,
                        WriteStorage<'a, PowerSwitch>,
                        WriteStorage<'a, PowerNode>,
                        ReadStorage<'a, Name>,
                        WriteStorage<'a, Door>,
                      );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut game_log,
            mut interact_intents,
            mut power_switches,
            mut power_nodes,
            names,
            mut doors,
        ) = data;

        let mut cleared_intents = Vec::new();

        //TODO: Add other interactions
        for (entity, interact_intent) in (&entities, &interact_intents).join() {
            macro_rules! handle_intent {
                ( $( $x:expr ),* ) => {
                    {
                        $(
                            if let Some(component) = $x.get_mut(interact_intent.target) {
                                if component.interaction_id == component.interaction_id {
                                    component.interact();
                
                                    if let Some(name) = names.get(entity) {
                                        game_log.entries.push(format!("{}: {}", name.name, interact_intent.interaction_description.clone()));
                                    }
                                    else {
                                        game_log.entries.push("Invalid intent".to_string());
                                    }
                                    cleared_intents.push(entity);
                                }
                            }

                            //If the interactable is powered, rebuild power state
                            if let Some(power_node) = power_nodes.get_mut(interact_intent.target) {
                                power_node.dirty = true;
                            }
                        )*
                    }
                };
            }

            handle_intent!(power_switches, doors);
        }

        //Clear intents
        for entity in cleared_intents.iter() {
            interact_intents.remove(*entity);
        }   
    }
}

pub fn get_entity_interactions(ecs: &World, entity: Entity) -> Vec<(usize, String, u32, u32)> {
    let names = ecs.read_storage::<Name>();

    let mut interactables = Vec::new();

    macro_rules! check_for_interactable {
        ($($typ:ty), *) => {
            {
                $(
                    let storage = ecs.read_storage::<$typ>();

                    if let Some(interactable) = storage.get(entity) {
                        let mut name = "{unknown}".to_string();
                
                        if let Some(entity_name) = names.get(entity) { name = entity_name.name.clone()}
                
                        interactables.push((
                            interactable.interaction_id,
                            format!("{} ({}): {}", name, interactable.state_description(), interactable.interaction_description),
                            entity.id(),
                            entity.id()
                        ));
                    }
                )*
            }
        };
    }

    //TODO: Add any other interactable components
    check_for_interactable!(PowerSwitch, Door);

    interactables
}
