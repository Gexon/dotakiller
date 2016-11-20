// принимаем входящее соединение от монстр-сервера

use tinyecs::*;

use std::net::TcpStream;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;

use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode};

use SERVER_IP;

//use ::ground::components::*;
use ::server::components::MonsterServerClass;


// структура приемник монстра
#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct MonsterExport {
    id: u64,
    x: f32,
    y: f32,
}


/// система монстр-сервер
pub struct MonsterServerSystem {
    server_data: MonsterServer,
}

impl MonsterServerSystem {
    pub fn new() -> MonsterServerSystem {
        let hostname: &str = SERVER_IP;
        let port: &str = "6658";
        let address = format!("{}:{}", hostname, port);

        let stream = TcpStream::connect(&*address).unwrap();
        info!("Монстер-сервер запущен. Подключен к главному серверу.");
        println!("Монстер-сервер запущен. Подключен к главному серверу.");

        let server = MonsterServer {
            stream: stream,
        };

        MonsterServerSystem {
            server_data: server,
        }
    }
}

// работа с сетью. передача данных клиентам.
impl System for MonsterServerSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterServerClass)
    }

    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, _world: &mut WorldHandle, _data: &mut DataList) {
        let monster_export: MonsterExport = self.server_data.read();
        println!("Приняли монстра {}", monster_export.id);
        for entity in entities {
            entity.remove_component::<MonsterServerClass>();
            entity.refresh();
        }
    }
}

/// Монстр-сервер
pub struct MonsterServer {
    stream: TcpStream,
}

impl MonsterServer {
    fn _write(&mut self) {
        //        let world = World {
        //            entities: vec![Entity { x: 0.0, y: 4.0 }, Entity { x: 10.0, y: 20.5 }]
        //        };

        // @AlexNav73 - спс за ссылку и помощь в освоении этой сериализации!
        let monster_export = MonsterExport {
            id: 0, x: 50f32, y: 50f32,
        };
        let encoded: Vec<u8> = encode(&monster_export, SizeLimit::Infinite).unwrap();

        let mut writer = BufWriter::new(&self.stream);
        let _ = writer.write(&encoded);
        writer.flush().unwrap();      // <------------ добавили проталкивание буферизованных данных в поток
    }

    fn read(&mut self) -> MonsterExport {
        let mut buf = vec![];
        let mut reader = BufReader::new(&self.stream);
        reader.read(&mut buf).unwrap();

        decode(&buf[..]).unwrap()
    }
}