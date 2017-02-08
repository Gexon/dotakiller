// работа с подключениями
use tinyecs::*;

use ::server::replicationserver::ReplicationServerSystem;
//use ::server::monsterserver::MonsterServerSystem;
use ::server::components::*;

mod replicationserver;
//mod monsterserver;
mod components;
mod connection;
mod commands;

pub fn init(dk_world: &mut World) {
    {
        // Создаем сервер:
        dk_world.set_system(ReplicationServerSystem::new());

        // создам сущность с компонентами сервер такой-то внутри.
        let mut entity_manager = dk_world.entity_manager();
        let entity = entity_manager.create_entity();

        entity.add_component(ReplicationServerClass);
        entity.refresh();
    }
}