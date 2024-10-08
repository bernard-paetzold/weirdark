use std::fs::{self, File};
use std::path::Path;

use bimap::BiMap;
use rltk::{Rltk, RGB};
use specs::error::NoError;
use specs::saveload::{DeserializeComponents, SerializeComponents, SimpleMarkerAllocator};

use specs::{
    saveload::{MarkedBuilder, SimpleMarker},
    Builder, World, WorldExt,
};
use specs::{Entity, Join};

use crate::entities::biology::Breather;
use crate::entities::intents::Initiative;
use crate::entities::power_components::{
    ControlPanel, ElectronicHeater, PowerNode, PowerSource, PowerSwitch, PoweredState, Wire,
};
use crate::entities::props::Cabinet;
use crate::{
    vectors::Vector3i, Illuminant, Name, Photometry, Player, Renderable, SerializationHelper,
    SerializeThis, Tile, Viewshed,
};
use crate::{
    Atmosphere, Blocker, Camera, Container, Door, Duct, EntityDirection, InContainer, Installed,
    Item, Prop, VisionBlocker, TERMINAL_HEIGHT, TERMINAL_WIDTH,
};

macro_rules! serialize_individually {
    ($ecs:expr, $ser:expr, $data:expr, $( $type:ty),*) => {
        $(
        SerializeComponents::<NoError, SimpleMarker<SerializeThis>>::serialize(
            &( $ecs.read_storage::<$type>(), ),
            &$data.0,
            &$data.1,
            &mut $ser,
        )
        .unwrap();
        )*
    };
}

pub fn save_game(ecs: &mut World) {
    let map_copy = ecs.get_mut::<super::map::Map>().unwrap().clone();

    let save_helper = ecs
        .create_entity()
        .with(SerializationHelper { map: map_copy })
        .marked::<SimpleMarker<SerializeThis>>()
        .build();

    {
        let data = (
            ecs.entities(),
            ecs.read_storage::<SimpleMarker<SerializeThis>>(),
        );

        let writer = File::create("./savegame.json").unwrap();
        let mut serializer = serde_json::Serializer::new(writer);

        serialize_individually!(
            ecs,
            serializer,
            data,
            Vector3i,
            Renderable,
            Player,
            Viewshed,
            Illuminant,
            Photometry,
            Name,
            Tile,
            Camera,
            PowerNode,
            PowerSource,
            PoweredState,
            PowerSwitch,
            Wire,
            ControlPanel,
            EntityDirection,
            Atmosphere,
            Breather,
            ElectronicHeater,
            Initiative,
            Item,
            Prop,
            InContainer,
            Installed,
            Container,
            Cabinet,
            VisionBlocker,
            Blocker,
            Door,
            Duct,
            SerializationHelper
        );
    }
    ecs.delete_entity(save_helper).expect("Crash on cleanup");
}

pub fn does_save_exist() -> bool {
    Path::new("./savegame.json").exists()
}

macro_rules! deserialize_individually {
    ($ecs:expr, $de:expr, $data:expr, $( $type:ty),*) => {
        $(
            println!("Deserializing {:?}", stringify!($type));
            match DeserializeComponents::<NoError, _>::deserialize(
                &mut ( &mut $ecs.write_storage::<$type>(), ),
                &mut $data.0, // entities
                &mut $data.1, // marker
                &mut $data.2, // allocater
                &mut $de,
            ) {
                Ok(_) => println!("Successfully deserialized {:?}", stringify!($type)),
                Err(e) => {
                    println!("Error deserializing {:?}: {:?}", stringify!($type), e);
                }
            }
        )*
    };
}

pub fn load_game(ecs: &mut World, ctx: &mut Rltk) {
    let progress_bar_width = TERMINAL_WIDTH / 2;
    {
        let mut to_delete = Vec::new();

        for e in ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            ecs.delete_entity(*del).expect("Deletion failed");
        }
    }

    ctx.set_active_console(2);
    ctx.cls();
    ctx.draw_bar_horizontal(
        TERMINAL_HEIGHT / 4,
        TERMINAL_HEIGHT / 2,
        progress_bar_width,
        0,
        progress_bar_width,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    let data = fs::read_to_string("./savegame.json").unwrap();
    let mut de = serde_json::Deserializer::from_str(&data);
    {
        let mut d = (
            &mut ecs.entities(),
            &mut ecs.write_storage::<SimpleMarker<SerializeThis>>(),
            &mut ecs.write_resource::<SimpleMarkerAllocator<SerializeThis>>(),
        );

        deserialize_individually!(
            ecs,
            de,
            d,
            Vector3i,
            Renderable,
            Player,
            Viewshed,
            Illuminant,
            Photometry,
            Name,
            Tile,
            Camera,
            PowerNode,
            PowerSource,
            PoweredState,
            PowerSwitch,
            Wire,
            ControlPanel,
            EntityDirection,
            Atmosphere,
            Breather,
            ElectronicHeater,
            Initiative,
            Item,
            Prop,
            InContainer,
            Installed,
            Container,
            Cabinet,
            VisionBlocker,
            Blocker,
            Door,
            Duct,
            SerializationHelper
        );
    }

    let mut delete_me: Option<Entity> = None;
    {
        let entities = ecs.entities();
        let helpers = ecs.read_storage::<SerializationHelper>();
        let players = ecs.read_storage::<Player>();
        let positions = ecs.read_storage::<Vector3i>();

        for (entity, helper) in (&entities, &helpers).join() {
            let mut worldmap = ecs.write_resource::<super::map::Map>();
            *worldmap = helper.map.clone();
            worldmap.entities = BiMap::new();
            delete_me = Some(entity);
        }

        for (entity, _player, position) in (&entities, &players, &positions).join() {
            let mut player_pos = ecs.write_resource::<Vector3i>();
            *player_pos = *position;

            let mut player_resource = ecs.write_resource::<Entity>();
            *player_resource = entity;
        }
    }

    if let Some(entity) = delete_me {
        let _ = ecs.delete_entity(entity);
    }
}

pub fn delete_save() {
    if Path::new("./savegame.json").exists() {
        std::fs::remove_file("./savegame.json").expect("Unable to delete file");
    }
}
