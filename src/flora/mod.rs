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

    for y in 0..10 {
        for count in 0..10 {
            // поручаем спавнеру, засумонить в наш мир пальму.
            // создаем спавнер
            let mut entity_manager = dk_world.entity_manager();
            let entity_spawner = entity_manager.create_entity();

            let _delta_x: f32 = count as f32;
            let delta_y: f32 = y as f32;
            entity_spawner.add_component(SpawnFlora { name: "palm", x: 10f32 + _delta_x, y: 10f32 + delta_y});
            entity_spawner.refresh();
        }
    }
}