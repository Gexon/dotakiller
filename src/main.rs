extern crate byteorder;
extern crate mio;
extern crate slab;
extern crate time;

#[macro_use] extern crate tinyecs;

#[macro_use] extern crate slog;
extern crate slog_stream;
extern crate slog_stdlog;
#[macro_use] extern crate log;


use std::net::SocketAddr;
use std::rc::Rc;
use mio::*;
use mio::tcp::*;

use tinyecs::*;

use server::*;
use utility::*;

mod server;
mod utility;
mod flora;
mod ground;



fn main() {

    utility::init();

    let hname: &str = "192.168.0.3";
    //let hname: &str = "194.87.237.144";
    let pname: &str = "6655";

    let address = format!("{}:{}", hname, pname);
    let addr = address.parse::<SocketAddr>().expect("Ошибка получения строки host:port");
    let sock = TcpListener::bind(&addr).expect("Ошибка биндинга адреса");

    // Создам объект опроса который будет использоваться сервером для получения событий
    let mut poll = Poll::new().expect("Ошибка создания опросника 'Poll'");

    // создаем мир компонентной системы.
    let mut dk_world = World::new();
    // Создаем сервер.
    let mut server = Server::new(sock, dk_world);

    // инициализация базы.
    //ground::init(&mut dk_world, &mut server);
    // инициализация растений.
    //flora::init(&mut dk_world);

    // Запускаем обработку событий, Poll - опросы событий хранятся внутри сервера.
    server.run(&mut poll).expect("Ошибка запуска сервера.");
}