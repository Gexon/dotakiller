// let a = ::module::components::Position::new();
use tinyecs::*;

use ::ground::components::SpawnPoint;
use ::flora::systems::*;

pub mod components;
mod systems;

pub fn init(dk_world: &mut World) {
    // добавляем в мир систему роста растений.
    dk_world.set_system(PlantGrowthSystem);
    dk_world.set_system(PlantReproductionSystem);
    dk_world.set_system(PlantDeadSystem);
    dk_world.set_system(PlantRemoveSystem);

    {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        entity_spawner.add_component(SpawnPoint { name: "palm", x: 10f32, y: 10f32});
        entity_spawner.refresh();
    }

    {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        entity_spawner.add_component(SpawnPoint { name: "palm", x: 11f32, y: 10f32});
        entity_spawner.refresh();
    }

    {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        entity_spawner.add_component(SpawnPoint { name: "palm", x: 12f32, y: 10f32});
        entity_spawner.refresh();
    }

    {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        entity_spawner.add_component(SpawnPoint { name: "palm", x: 13f32, y: 10f32});
        entity_spawner.refresh();
    }

    {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        entity_spawner.add_component(SpawnPoint { name: "palm", x: 14f32, y: 10f32});
        entity_spawner.refresh();
    }

    {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        entity_spawner.add_component(SpawnPoint { name: "palm", x: 10f32, y: 15f32});
        entity_spawner.refresh();
    }

    {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        entity_spawner.add_component(SpawnPoint { name: "palm", x: 11f32, y: 15f32});
        entity_spawner.refresh();
    }

    {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        entity_spawner.add_component(SpawnPoint { name: "palm", x: 12f32, y: 15f32});
        entity_spawner.refresh();
    }

    {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        entity_spawner.add_component(SpawnPoint { name: "palm", x: 13f32, y: 15f32});
        entity_spawner.refresh();
    }

    {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        entity_spawner.add_component(SpawnPoint { name: "palm", x: 14f32, y: 15f32});
        entity_spawner.refresh();
    }

    {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        entity_spawner.add_component(SpawnPoint { name: "palm", x: 10f32, y: 25f32});
        entity_spawner.refresh();
    }

    {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        entity_spawner.add_component(SpawnPoint { name: "palm", x: 15f32, y: 25f32});
        entity_spawner.refresh();
    }

    {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        entity_spawner.add_component(SpawnPoint { name: "palm", x: 20f32, y: 25f32});
        entity_spawner.refresh();
    }

    {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        entity_spawner.add_component(SpawnPoint { name: "palm", x: 25f32, y: 25f32});
        entity_spawner.refresh();
    }

    {
        // поручаем спавнеру, засумонить в наш мир пальму.
        // создаем спавнер
        let mut entity_manager = dk_world.entity_manager();
        let entity_spawner = entity_manager.create_entity();

        entity_spawner.add_component(SpawnPoint { name: "palm", x: 30f32, y: 25f32});
        entity_spawner.refresh();
    }
}