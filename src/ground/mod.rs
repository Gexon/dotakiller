// основные компоненты и системы, общие для всех, буду хранить тут.
// так же тут буду размещать компоненты и системы по терраморфингу.
use tinyecs::*;
use time::PreciseTime;

use ::ground::components::*;
use ::ground::systems::*;

pub mod components;
pub mod systems;

pub fn init(dk_world: &mut World) {
    // добавляем в мир систему спавна.
    dk_world.set_system(SpawnSystem);
    dk_world.set_system(WindDirectionSystem);

    {
        // вносим в этот мир немного земли
        let mut entity_manager = dk_world.entity_manager();
        let entity = entity_manager.create_entity();

        entity.add_component(ClassGround);
        entity.add_component(WorldMap { flora_x: [0; 140], flora_y: [0; 140] });
        entity.add_component(WindDirection { direction: 0, start: PreciseTime::now() });
        entity.add_component(WorldLastId { flora_id: 0 });
        entity.refresh();
    }
}