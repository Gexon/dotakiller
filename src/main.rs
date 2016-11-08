extern crate byteorder;
extern crate mio;
extern crate slab;
extern crate time;

#[macro_use] extern crate tinyecs;

#[macro_use] extern crate slog;
extern crate slog_stream;
extern crate slog_stdlog;
#[macro_use] extern crate log;



use tinyecs::*;

mod server;
mod utility;
mod flora;
mod ground;

//const  WORLD_SPEED: &'static i32 = 1;
static WORLD_SPEED: i64 = 1;

fn main() {
    utility::init(); // запускаем логгер.
    let mut dk_world = World::new(); // создаем мир компонентной системы.
    server::init(&mut dk_world);    // инициализация сервера.
    ground::init(&mut dk_world);    // инициализация основ мира.
    flora::init(&mut dk_world);     // инициализация растений.
    loop {dk_world.update()}        // основной цикл ECS.
}