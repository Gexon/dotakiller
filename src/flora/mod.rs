// let a = ::module::components::Position::new();
use tinyecs::*;

use ::ground::components::SpawnPoint;
use ::ground::components::Position;
use ::flora::systems::PlantGrowth;

pub mod components;
mod systems;

pub fn init(dk_world: &mut World) {
    // добавляем в мир систему роста растений.
    dk_world.set_system(PlantGrowth);
    //dk_world.set_system(PlantReproduction);

    {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity = entity_manager.create_entity();

        entity.add_component(SpawnPoint { name: "palm" });
        entity.add_component(Position { x: 0f32, y: 0f32 });
        entity.refresh();
    }
}