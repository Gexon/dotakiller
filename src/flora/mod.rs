// let a = ::module::components::Position::new();
use tinyecs::*;

use ::ground::components::SpawnFlora;
use ::flora::systems::*;

pub mod components;
mod systems;

pub fn init(dk_world: &mut World) {
    // добавляем в мир систему роста растений.
    dk_world.set_system(PlantGrowthSystem);
    dk_world.set_system(PlantReproductionSystem);
    dk_world.set_system(PlantDeadSystem);
    dk_world.set_system(PlantRemoveSystem);

    for count in 0..100 {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        let delta: f32 = count as f32;
        entity_spawner.add_component(SpawnFlora { name: "palm", x: 1f32 + delta, y: 1f32 + delta });
        entity_spawner.refresh();
    }
}