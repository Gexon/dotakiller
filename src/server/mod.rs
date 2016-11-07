// работа с подключениями
use tinyecs::*;

use ::server::systems::*;

mod systems;
mod connection;
mod commands;

pub fn init(dk_world: &mut World) {
    {
        // Создаем сервер.
        dk_world.set_system(ServerSystem::new());

        // создам сущность с сервером внутри.
        let mut entity_manager = dk_world.entity_manager();
        let entity = entity_manager.create_entity();

        // рудимент.
        entity.add_component(ServerClass);
        entity.refresh();
    }
}