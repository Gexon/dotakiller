// принимаем входящее соединение от монстр-сервера

use tinyecs::*;

use std::net::{TcpStream, TcpListener};
use std::io;

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
struct MonsterImport {
    id: i64,
    x: f32,
    y: f32,
}

// структура отдачник монстра
#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct MonsterExport {
    id: u64,
    damage: u64,
}

// массив/вектор принятых монстров
#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct MonsterArrayImport {
    entities: Vec<MonsterImport>
}

// массив/вектор отдающихся монстров
#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct MonsterArrayExport {
    entities: Vec<MonsterExport>
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
        let listener = TcpListener::bind(&*address).expect("Ошибка биндинга адреса");
        info!("Порт открыт для приема подключений Монстер-сервера.");

        info!("Ожидаем подключения Монстер-сервера.");
        println!("Ожидаем подключения Монстер-сервера.");
        let stream = t!(listener.accept()).0;
        t!(stream.set_read_timeout(Some(Duration::from_millis(1000))));
        info!("Приняли подключение Монстер-сервера.");
        println!("Приняли подключение Монстер-сервера.");

        // создаем читателя
        let writer_stream = stream.try_clone().unwrap();

        let server = MonsterServer {
            reader: BufReader::new(stream),
            _writer: BufWriter::new(writer_stream),
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
        //println!("Готовимся принять данные от Монстра-сервера.");
        let monster_array_import = match self.server_data.read() {
            Ok(data) => data,
            Err(_) => {
                //println!("Ошибка получения данных {}", e);
                return},
        };

        if !monster_array_import.entities.is_empty() {
            let monster_entities = monster_array_import.entities;
            for monster in monster_entities {
                let in_monster: MonsterImport = monster;
                println!("Приняли монстра {}, x {}, y {}", in_monster.id, in_monster.x, in_monster.y);
            }
        } else { println!("От Монстра-сервера пришли пустые данные."); }

        for _entity in entities {
            //entity.remove_component::<MonsterServerClass>();
            //entity.refresh();
        }
    }
}


/// Монстр-сервер поток
pub struct MonsterServer {
    // stream: TcpStream,
    reader: BufReader<TcpStream>,
    _writer: BufWriter<TcpStream>,
}

impl MonsterServer {
    fn _write(&mut self) {
        // @AlexNav73 - спс за ссылку и помощь в освоении этой сериализации!
        let monster_export = MonsterExport {
            id: 0, damage: 0
        };
        let encoded: Vec<u8> = encode(&monster_export, SizeLimit::Infinite).unwrap();

        //let mut writer = BufWriter::new(&self.stream);
        let _ = self._writer.write(&encoded);
        self._writer.flush().unwrap();      // <------------ добавили проталкивание буферизованных данных в поток
    }

    fn read(&mut self) -> io::Result<MonsterArrayImport> {
        // готовим вектор для примема размера входящих данных
        let mut buf_len = [0u8; 8];
        // принимаем сообщение о размере входящих данных.
        let bytes = match self.reader.read(&mut buf_len) {
            Ok(n_read) => {
                //let s = str::from_utf8(&buf_len[..]).unwrap();
                //println!("Содержимое сообщения о длине входящих данных:{:?}, количество считанных байт:{}", s, n_read);
                n_read
            },
            Err(e) => {
                return Err(e);
            }
        };

        if bytes < 8 {
            warn!("Ошибка. Сообщение о длине входящих данных меньше 8 байт и равно: {} bytes", bytes);
            println!("Ошибка. Сообщение о длине входящих данных меньше 8 байт и равно: {} bytes", bytes);
            //return Err("Some error message");
        }

        // превращаем в нормальный вид длину входящих данных.
        let msg_len = BigEndian::read_u64(buf_len.as_ref());
        let msg_len = msg_len as usize;
        debug!("Ожидаемая длина сообщения {}", msg_len);
        println!("Ожидаемая длина сообщения {}", msg_len);
        // подготавливаем вектор для принимаемых данных.
        let mut recv_buf: Vec<u8> = Vec::with_capacity(msg_len);
        unsafe { recv_buf.set_len(msg_len); }
        // прием данных
        match self.reader.read(&mut recv_buf) {
            Ok(n) => {
                debug!("CONN : считано {} байт", n);
                println!("CONN : считано {} байт", n);
                if n < msg_len as usize {
                    println!("Не осилил достаточно байт");
                }
                Ok(decode(&recv_buf[..]).unwrap())
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}