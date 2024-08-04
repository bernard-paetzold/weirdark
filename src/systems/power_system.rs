use specs::{prelude::*, storage::GenericReadStorage};

use crate::{
    vectors::Vector3i, Illuminant, Map, Photometry, Player, Power, PowerSwitch, Viewshed
};

pub struct PowerSystem {}

impl<'a> System<'a> for PowerSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteStorage<'a, Power>,
        ReadStorage<'a, PowerSwitch>,
        WriteStorage<'a, Illuminant>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            mut power,
            power_switches,
            mut illuminants,
            entities
        ) = data;

        //Align powered state with switches
        for (power, power_switch) in (&mut power, &power_switches).join() {
            power.powered = power_switch.on;
        }

        //Align powered components with powered state
        for (power, entity) in (&mut power, &entities).join() {
            //Illuminant
            if let Some(illuminant) = illuminants.get_mut(entity) {
                illuminant.set_state(power.powered);
            }

            //TODO: Add any other powered systems here
        }
    }
}
