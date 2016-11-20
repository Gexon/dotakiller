// принимаем входящее соединение от монстр-сервера

use tinyecs::*;

use std::net::{TcpStream, TcpListener};
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::time::Duration;
use std::str;

use byteorder::{ByteOrder, BigEndian};

use bincode::SizeLimit;
use bincode::rustc_serialize::{encode, decode};

use SERVER_IP;

//use ::ground::components::*;
use ::server::components::MonsterServerClass;

macro_rules! t {
        ($e:expr) => {
            match $e {
                Ok(t) => t,
                Err(e) => panic!("received error for `{}`: {}", stringify!($e), e),
            }
        }
    }

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
        //let listener = TcpListener::bind(&*address).unwrap();
        let listener = TcpListener::bind(&*address).expect("Ошибка биндинга адреса");
        info!("Порт открыт для приема подключений Монстер-сервера.");

        info!("Ожидаем подключения Монстер-сервера.");
        println!("Ожидаем подключения Монстер-сервера.");
        let stream = t!(listener.accept()).0;
        t!(stream.set_read_timeout(Some(Duration::from_millis(1000))));
        info!("Приняли подключение Монстер-сервера.");
        println!("Приняли подключение Монстер-сервера.");

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
        println!("Готовимся принять данные от Монстра-сервера.");
        let monster_export: MonsterExport = self.server_data.read();
        println!("Данные от Монстра-сервера получены.");
        println!("Приняли монстра {}", monster_export.id);
        for entity in entities {
            entity.remove_component::<MonsterServerClass>();
            entity.refresh();
        }
    }
}


/// Монстр-сервер поток
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
        // создаем читателя
        let mut reader = BufReader::new(&self.stream);
        // готовим вектор для примема размера входящих данных
        let mut buf_len = [0u8; 8];
        // принимаем данные
        let bytes = match reader.read(&mut buf_len) {
            Ok(n) => {
                let s = str::from_utf8(&buf_len[..]).unwrap();
                println!("Содержимое сообщения о длине входящих данных:{}, количество считанных байт:{}", s, n);
                n
            },
            Err(_) => {
                0
            }
        };
        if bytes < 8 {
            warn!("Ошибка. Сообщение о длине входящих данных меньше 8 байт и равно: {} bytes", bytes);
            println!("Ошибка. Сообщение о длине входящих данных меньше 8 байт и равно: {} bytes", bytes);
        }
        // превращаем в нормальный вид длину входящих данных.
        let msg_len = BigEndian::read_u64(buf_len.as_ref());
        let msg_len = msg_len as usize;
        debug!("Ожидаемая длина сообщения {}", msg_len);
        println!("Ожидаемая длина сообщения {}", msg_len);
        // подготавливаем вектор для принимаемых данных.
        let mut recv_buf: Vec<u8> = Vec::with_capacity(msg_len);
        unsafe { recv_buf.set_len(msg_len); }
        //let stream_ref = <TcpStream as Read>::by_ref(&self.stream);
        //match self.stream.take(msg_len as u64).read(&mut recv_buf) {
        // прием данных
        match reader.read(&mut buf_len) {
            Ok(n) => {
                debug!("CONN : считано {} байт", n);
                println!("CONN : считано {} байт", n);
                if n < msg_len as usize {
                    println!("Не осилил достаточно байт");
                }
                decode(&recv_buf[..]).unwrap()
            }
            Err(_) => {
                panic!("Неудалось считать буфер для сокета");
            }
        }
    }
}