// основные компоненты и системы, общие для монстра, буду хранить тут.

use tinyecs::*;
use time::{PreciseTime};

use ::ground::components::SpawnMonster;
use ::monster::systems::*;

pub mod components;
pub mod systems;

// todo (Генетические алгоритмы) передача кода behaviour tree потомкам монстра.
// todo запилить мутации кода behaviour tree, либо скрещивания как у пчел с майнкрафта.
/// инициализация. создаем первого монстра.
pub fn init(monster_world: &mut World) {
    monster_world.set_system(SelectorSystem);
    monster_world.set_system(BehaviorSystem { behavior_time: PreciseTime::now() });
    monster_world.set_system(EventSystem { event_time: PreciseTime::now(), event_last: 0 });
    monster_world.set_system(BioSystems { bios_time: PreciseTime::now() });

    for count in 0..1 {
        // поручаем спавнеру, засумонить в наш мир первого монстра!
        // создаем спавнер
        let mut entity_manager = monster_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        let delta: f32 = count as f32;
        entity_spawner.add_component(SpawnMonster { name: "monster", x: 1f32 + delta, y: 1f32 + delta });
        entity_spawner.refresh();
        //break;
    }
}