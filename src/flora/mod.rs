// let a = ::module::components::Position::new();
use tinyecs::*;

use ::ground::components::SpawnPoint;
use ::flora::systems::PlantGrowth;
use ::flora::systems::PlantReproduction;

pub mod components;
mod systems;

pub fn init(dk_world: &mut World) {
    // добавляем в мир систему роста растений.
    dk_world.set_system(PlantGrowth);
    dk_world.set_system(PlantReproduction);

    {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        entity_spawner.add_component(SpawnPoint { name: "palm", x: 0f32, y: 0f32});
        entity_spawner.refresh();
    }
}