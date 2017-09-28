extern crate byteorder;
extern crate bytes;
extern crate mio;
extern crate slab;
extern crate time;

// tokio
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

#[macro_use]
extern crate tinyecs;
//use time::{PreciseTime, Duration};

#[macro_use]
extern crate slog;
extern crate slog_stream;
extern crate slog_stdlog;
//#[macro_use]
extern crate log;

extern crate bincode;
extern crate rustc_serialize;

use tinyecs::*;

mod server;
mod utility;
mod flora;
mod monster;
mod ground;

// port 6655
const SERVER_IP: &'static str = "192.168.0.3";
//const  SERVER_IP: &'static str = "194.87.237.144";
static GROUND_SPEED: i64 = 10;
static FLORA_SPEED: i64 = 100;//1000
static MONSTER_SPEED: i64 = 4;//10
static GROUND_SIZE: u32 = 140;
// 1_000_000ns = 1ms
//static TICK_MAX_TIME: u64 = 50;//Duration::milliseconds(50);


fn main() {
    utility::init(); // запускаем логгер.
    let mut dk_world = World::new(); // создаем мир компонентной системы.
    server::init(&mut dk_world);    // инициализация сервера.
    ground::init(&mut dk_world);    // инициализация основ мира.
    flora::init(&mut dk_world);     // инициализация растений.
    monster::init(&mut dk_world);     // инициализация монстров.
    //let mut event_time: std::time::Instant;//PreciseTime;
    //let max_time = std::time::Duration::from_millis(TICK_MAX_TIME);
    loop {
        // засекаю время.
        //event_time = std::time::Instant::now();//PreciseTime::now();
        dk_world.update(); // основной цикл ECS.
        // смотрю сколько потратил.
        // вычисляю дельту до if т.к. сервер крашится при выполнении sleep
        //let delta = max_time - std::time::Instant::now().duration_since(event_time);
        //if delta > std::time::Duration::new(0, 0) {
        // надо спать разницу между time_max и временем затраченным на полезную работу
        //std::thread::sleep(delta);
        //} else {
        //trace!("Main loop > TICK_MAX_TIME");
        std::thread::sleep(std::time::Duration::from_millis(1));
        //}
    }
}