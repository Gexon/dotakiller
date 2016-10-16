// Взято отсюдо https://github.com/hjr3/mob

extern crate byteorder;
extern crate mio;
extern crate slab;

#[macro_use] extern crate log;
extern crate env_logger;

mod server;
mod connection;

use std::net::SocketAddr;

use mio::*;
use mio::tcp::*;

use server::*;


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

    // Создаем сервер и запускаем обработку событий, Poll.
    // Опросы событий хранятся внутри сервера.
    let mut server = Server::new(sock);
    server.run(&mut poll).expect("Ошибка запуска сервера.");
}
