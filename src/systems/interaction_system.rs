use std::collections::HashMap;

use specs::prelude::*;

use crate::{gamelog::GameLog, vectors::Vector3i, InteractIntent, Name, OpenContainer, PowerSwitch};

pub struct InteractionSystem {}

impl<'a> System<'a> for InteractionSystem {
    type SystemData = ( ReadExpect<'a, Entity>,
                        WriteExpect<'a, GameLog>,
                        WriteStorage<'a, InteractIntent>,
                        WriteStorage<'a, PowerSwitch>,
                        WriteStorage<'a, OpenContainer>,
                      );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut entities,
            mut game_log,
            mut interact_intents,
            mut power_switches,
            mut containers,
        ) = data;


        for (interact_intent, power_switch) in (&interact_intents, &mut power_switches).join() {
            if power_switch.interaction_id == interact_intent.interaction_id {
                power_switch.toggle();
            }
            interact_intents.remove(interact_intent.target);
            break;
        }

        for (interact_intent, container) in (&interact_intents, &mut containers).join() {
            if container.interaction_id == interact_intent.interaction_id {
                container.toggle();
            }
            interact_intents.remove(interact_intent.target);
            break;
        }
        
    }
}