// принимаем входящее соединение от монстр-сервера

use tinyecs::*;

use std::net::{TcpStream, TcpListener};
use std::io;

use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::time::Duration;
use std::str;

use byteorder::{ByteOrder, BigEndian};

use bincode::SizeLimit; // @AlexNav73 - спс за ссылку и помощь в освоении этой сериализации!
use bincode::rustc_serialize::{encode, decode};

use SERVER_IP;

use ::ground::components::MonsterClass;
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
    p_type: u8,
    id: i64,
    x: f32,
    y: f32,
}

// структура отдачник монстра
#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct MonsterExport {
    p_type: u8,
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
            writer: BufWriter::new(writer_stream),
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

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![MonsterClass]]
    }

    fn process_all(&mut self, _entities: &mut Vec<&mut Entity>, _world: &mut WorldHandle, data: &mut DataList) {
        let monsters = data.unwrap_all();
        let monster_exist = false;

        // Тут все на соплях, если вдрух и этот и тот сервер ждут сообщений с сокета - тогда ппц.
        /// Внимание! Важно, не менять порядок приема-передачи пакетов, иначе писец.
        // Передаем idle Монстр-серверу. -----------------------------------------------------------
        let monster_export = MonsterExport {
            p_type: 0, id: 0, damage: 1 // p_type = 0 idle
        };
        let monster_array = MonsterArrayExport {
            entities: vec![monster_export]
        };
        self.server_data.write(monster_array);

        // Принимаем данные от Монстра-сервера. ----------------------------------------------------
        let monster_array_import = match self.server_data.read() {
            Ok(data) => data, // monster_array_import - это структура, внутри поле entities c вектором
            Err(e) => {
                println!("Ошибка получения данных {}", e);
                return
            },
        };
        // Конец блока приема-передачи, можно расслабить булки. ------------------------------------

        // обрабатываем полученные данные
        if !monster_array_import.entities.is_empty() {
            let monster_entities = monster_array_import.entities; // entities - это один(!) вектор. entities: Vec<MonsterImport>
            for monster in monster_entities {
                let in_monster: MonsterImport = monster;
                if in_monster.p_type == 0 {
                    //println!("Приняли idle"); //TODO переделать, иначе будет работать со скоростью монстр-сервера.
                } else {

                    println!("Приняли монстра {}, x {}, y {}", in_monster.id, in_monster.x, in_monster.y);
                }
            }
        } else { println!("От Монстра-сервера пришли пустые данные."); }
    }
}


/// Монстр-сервер поток
pub struct MonsterServer {
    // stream: TcpStream,
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl MonsterServer {
    fn write(&mut self, monster_array: MonsterArrayExport) {
        let encoded: Vec<u8> = encode(&monster_array, SizeLimit::Infinite).unwrap();

        let len = encoded.len();
        let mut send_buf = [0u8; 8];
        BigEndian::write_u64(&mut send_buf, len as u64);

        let _ = self.writer.write(&send_buf);
        let _ = self.writer.write(&encoded);
        self.writer.flush().expect("Ошибка передачи данных, возможно отвалился монстр-сервер");      // <------------ добавили проталкивание буферизованных данных в поток
        //self._writer.flush().unwrap();      // <------------ добавили проталкивание буферизованных данных в поток
        //println!("Длина отправленных данных {}", len);
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
        //debug!("Ожидаемая длина сообщения {}", msg_len);
        //println!("Ожидаемая длина сообщения {}", msg_len);
        // подготавливаем вектор для принимаемых данных.
        let mut recv_buf: Vec<u8> = Vec::with_capacity(msg_len);
        unsafe { recv_buf.set_len(msg_len); }
        // прием данных
        match self.reader.read(&mut recv_buf) {
            Ok(n) => {
                //debug!("CONN : считано {} байт", n);
                //println!("CONN : считано {} байт", n);
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