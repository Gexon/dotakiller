extern crate byteorder;
extern crate mio;
extern crate slab;
extern crate time;

#[macro_use]
extern crate tinyecs;
use time::{PreciseTime, Duration};

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
static WORLD_SPEED: i64 = 10;

fn main() {
    utility::init(); // запускаем логгер.
    let mut dk_world = World::new(); // создаем мир компонентной системы.
    server::init(&mut dk_world);    // инициализация сервера.
    ground::init(&mut dk_world);    // инициализация основ мира.
    flora::init(&mut dk_world);     // инициализация растений.
    monster::init(&mut dk_world);     // инициализация монстров.
    let mut event_time: PreciseTime;
    loop {
        // засекаю время.
        event_time = PreciseTime::now();
        dk_world.update(); // основной цикл ECS.
        // смотрю сколько потратил.
        if event_time.to(PreciseTime::now()) > Duration::milliseconds(100) {
            // если update() > 100ms то ворнинг.
            trace!("Main loop > 100ms");
        } else {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        // пауза должна составить 200мс. в штатном режиме.
        // при перегрузке 100мс.
    }
}