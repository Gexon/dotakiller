// принимаем входящее соединение от монстр-сервера

use tinyecs::*;

use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;

use ::flora::components::FloraClass;



/// система монстр-сервер
pub struct MonsterServerSystem {
    server_data: MonsterServer,
}

impl MonsterServerSystem {}

// работа с сетью. передача данных клиентам.
impl System for MonsterServerSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(FloraClass)
    }

    fn process_all(&mut self, _entities: &mut Vec<&mut Entity>, _world: &mut WorldHandle, _data: &mut DataList) {}
}

/// Монстр-сервер
pub struct MonsterServer {
    stream: TcpStream,
}

impl MonsterServer {
    fn execute() {
        loop {
            // перенести в систему
        }
    }
}