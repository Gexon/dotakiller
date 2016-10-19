// let a = ::module::components::Position::new();
use tinyecs::*;

use ::ground::components::SpawnPoint;
use ::ground::components::Position;
use ::flora::components::*;

mod components;
mod systems;

pub fn init(dk_world: &mut World) {
    {
        // поручаем спавнеру, засумонить в наш мир пальму.
        let mut entity_manager = dk_world.entity_manager();
        let entity = entity_manager.create_entity();

        entity.add_component(SpawnPoint { name: "Palm", count: 1 });
        entity.add_component(Position { x: 0f32, y: 0f32 });
        entity.add_component(FloraClass);
        entity.refresh();
    }
}