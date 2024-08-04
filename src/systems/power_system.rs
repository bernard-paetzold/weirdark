use specs::prelude::*;

use crate::{
    Illuminant, Map, Photometry, Power, PowerSwitch
};

pub struct PowerSystem {}

impl<'a> System<'a> for PowerSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteStorage<'a, Power>,
        ReadStorage<'a, PowerSwitch>,
        WriteStorage<'a, Illuminant>,
        WriteStorage<'a, Photometry>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            _map,
            mut power,
            power_switches,
            mut illuminants,
            mut photometrics,
            entities
        ) = data;

        //Align powered on state with switches
        for (power, power_switch) in (&mut power, &power_switches).join() {
            power.on = power_switch.on;
        }

        //Align powered components with powered state
        for (power, entity) in (&mut power, &entities).join() {

            let power_state = power.powered && power.on;
            //Illuminant
            if let Some(illuminant) = illuminants.get_mut(entity) {
                illuminant.set_state(power_state);

                if let Some(photometry) = photometrics.get_mut(entity) {
                    photometry.dirty = true;
                }
            }

            //TODO: Add any other powered systems here
        }
    }
}
