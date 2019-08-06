// основные компоненты и системы, общие для аборигенов, буду хранить тут.

use tinyecs::*;

use ::ground::components::SpawnAborigen;

pub mod components;
pub mod systems;
pub mod aborigen_goap;


/// инициализация. создаем первого аборигена.
pub fn init(aborigen_world: &mut World) {

    for count in 0..2 {
        // поручаем спавнеру, засумонить в наш мир первого монстра!
        // создаем спавнер
        let mut entity_manager = aborigen_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        let delta: f32 = count as f32;
        entity_spawner.add_component(SpawnAborigen { name: "aborigen", x: 20f32 + delta, y: 10f32 + delta });
        entity_spawner.refresh();
    }
}