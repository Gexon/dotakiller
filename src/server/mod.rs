// работа с подключениями
use tinyecs::*;

use std::thread;
//use std::rc::Rc;
//use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio_core::reactor::Core;

//use ::server::replicationserver::ReplicationServerSystem;
use ::server::treplicationserver::ReplicationServerSystem;
use ::server::treplicationserver::ReplicationServer;
//use ::server::monsterserver::MonsterServerSystem;
use ::server::components::*;

mod treplicationserver;
//mod replicationserver;
//mod monsterserver;
mod components;
//mod connection;
mod commands;
mod proto;

pub fn init(dk_world: &mut World) {
    {
        let connections_info = Arc::new(Mutex::new(HashMap::new()));
        let connections_recive = Arc::new(Mutex::new(HashMap::new()));
        let connections = Arc::new(Mutex::new(HashMap::new()));
        let connections_info_clone = connections_info.clone();
        let connections_recive_clone = connections_recive.clone();
        let connections_clone = connections.clone();
        // Создаем сервер:
        thread::spawn(|| {
            let core = Core::new().unwrap();
            //let connections = Rc::new(RefCell::new(HashMap::new()));
            //let connections_recive = Rc::new(RefCell::new(HashMap::new()));
            //let connections_info = Rc::new(RefCell::new(HashMap::new()));
            let mut replication_server = ReplicationServer {
                core: core,
                connections: connections,
                connections_recive: connections_recive,
                connections_info: connections_info,
            };
            replication_server.run();
        });


        dk_world.set_system(ReplicationServerSystem {
            connections: connections_clone,
            connections_recive: connections_recive_clone,
            connections_info: connections_info_clone,
        });

        // создам сущность с компонентами сервер такой-то внутри.
        let mut entity_manager = dk_world.entity_manager();
        let entity = entity_manager.create_entity();

        entity.add_component(ReplicationServerClass);
        entity.refresh();
    }
}