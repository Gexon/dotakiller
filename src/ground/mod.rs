// основные компоненты и системы, общие для всех, буду хранить тут.
// так же тут буду размещать компоненты и системы по терраморфингу.
use tinyecs::*;
use time::PreciseTime;
use std::collections::HashMap;

//use ::utility::map::Map;
use ::utility::enums::Direction;
//use ::utility::map::Size;
use ::ground::components::*;
use ::ground::systems::*;
use ::ground::event_systems::*;


pub mod components;
pub mod systems;
pub mod event_systems;

pub fn init(dk_world: &mut World) {
    // добавляем в мир систему спавна.
    dk_world.set_system(SpawnFloraSystem);
    dk_world.set_system(SpawnMonsterSystem);
    dk_world.set_system(SpawnAborigenSystem);
    dk_world.set_system(WindDirectionSystem);
    dk_world.set_system(FloraEventSystem);
    dk_world.set_system(MonsterEventSystem);
    //dk_world.set_system(MonsterMapSystem);

    {
        // вносим в этот мир немного земли
        let mut entity_manager = dk_world.entity_manager();
        let entity = entity_manager.create_entity();

        entity.add_component(ClassGround);
        entity.add_component(WorldMap {
            flora: HashMap::new(),
        });
        entity.add_component(WindDirection { direction: Direction::North, start: PreciseTime::now() });
        entity.add_component(WorldLastId {
            flora_id: 0,
            monster_id: 0,
            aborigen_id: 0,
        });
        entity.add_component(EventsTo::new());
        entity.refresh();
    }
}