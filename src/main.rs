extern crate byteorder;
extern crate mio;
extern crate slab;
extern crate time;

#[macro_use]
extern crate tinyecs;
//use time::{PreciseTime, Duration};

#[macro_use]
extern crate slog;
extern crate slog_stream;
extern crate slog_stdlog;
#[macro_use]
extern crate log;

extern crate bincode;
extern crate rustc_serialize;

use tinyecs::*;

mod server;
mod utility;
mod flora;
mod monster;
mod ground;

const SERVER_IP: &'static str = "192.168.0.131";
//const  SERVER_IP: &'static str = "194.87.237.144";
static WORLD_SPEED: i64 = 1;
// 1_000_000ns = 1ms
static TICK_MAX_TIME: u64 = 50;//Duration::milliseconds(50);

fn main() {
    utility::init(); // запускаем логгер.
    let mut dk_world = World::new(); // создаем мир компонентной системы.
    server::init(&mut dk_world);    // инициализация сервера.
    ground::init(&mut dk_world);    // инициализация основ мира.
    flora::init(&mut dk_world);     // инициализация растений.
    monster::init(&mut dk_world);     // инициализация монстров.
    let mut event_time: std::time::Instant;//PreciseTime;
    let max_time = std::time::Duration::from_millis(TICK_MAX_TIME);
    loop {
        // засекаю время.
        event_time = std::time::Instant::now();//PreciseTime::now();
        dk_world.update(); // основной цикл ECS.
        // смотрю сколько потратил.
        if std::time::Instant::now().duration_since(event_time) < max_time {
            // надо спать разницу между time_max и временем затраченным на полезную работу
            std::thread::sleep(max_time - std::time::Instant::now().duration_since(event_time));
        } else {
            trace!("Main loop > TICK_MAX_TIME");
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
}