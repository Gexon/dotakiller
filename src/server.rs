use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::prelude::*;
use std::io::BufReader;

pub struct GameServer {
    addres: String,
}

impl GameServer {
    pub fn new(hostname: &str, port: &str) -> GameServer {
        let addres = format!("{}:{}", hostname, port);

        GameServer {
            addres: addres,
        }
    }

    pub fn start(&self) -> bool {

        println!("Дота должна умереть!");
        println!("Адрес сервера: {}", self.addres);


        let listener = match TcpListener::bind(&*self.addres) {
            Ok(data) => data,
            Err(e) => {
                println!("Ошибка открытия порта: {}", e);
                return false;
            },
        };

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    thread::spawn(move || {
                        handle_client(&mut stream)
                    });
                },
                Err(e) => {
                    println!("Ошибка при запуске сервера: {}", e);
                    return false;
                }
            }
        }

        fn handle_client(stream: &mut TcpStream) {
            let mut reader = BufReader::new(stream);
            println!("Подключен неизвестный клиент, ip: {}:{}",
                     reader.get_ref().local_addr().unwrap().ip(),
                     reader.get_ref().local_addr().unwrap().port());

            loop {
                let mut data = String::new();
                match reader.read_line(&mut data).unwrap() {
                    0 => {
                        println!("Неизвестный клиент был отключен, ip: {}:{}",
                                 reader.get_ref().local_addr().unwrap().ip(),
                                 reader.get_ref().local_addr().unwrap().port());
                        return;
                    },
                    _ => (),
                }

                println!("Передача данных: {}", data);
            }
        }

        drop(listener);

        true
    }
}