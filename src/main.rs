extern crate byteorder;
extern crate mio;
extern crate slab;
extern crate time;

#[macro_use] extern crate tinyecs;

#[macro_use] extern crate log;
extern crate env_logger;

use std::net::SocketAddr;

use mio::*;
use mio::tcp::*;

use tinyecs::*;

use server::*;

mod server;
mod connection;
mod commands;
mod dbqury;
mod flora;
mod ground;

fn main() {
    let hname: &str = "192.168.0.3";
    //let hname: &str = "194.87.237.144";
    let pname: &str = "6655";

    // Регистрируем логгер для дебага и статистики.
    env_logger::init().expect("Ошибка инициализации логгера");

    let address = format!("{}:{}", hname, pname);
    let addr = address.parse::<SocketAddr>().expect("Ошибка получения строки host:port");
    let sock = TcpListener::bind(&addr).expect("Ошибка биндинга адреса");

    // Создам объект опроса который будет использоваться сервером для получения событий
    let mut poll = Poll::new().expect("Ошибка создания опросника 'Poll'");

    // запуск компонентной системы.
    let mut dk_world = World::new();
    // инициализация базы.
    ground::init(&mut dk_world);
    // инициализация растений.
    flora::init(&mut dk_world);

    // Создаем сервер и запускаем обработку событий, Poll.
    // Опросы событий хранятся внутри сервера.
    let mut server = Server::new(sock, dk_world);
    server.run(&mut poll).expect("Ошибка запуска сервера.");
}