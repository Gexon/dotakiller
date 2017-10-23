// тут будем хранить весь хлам

pub mod logger;
pub mod dbqury;
pub mod map;
pub mod enums;

pub fn init(){
    logger::init();
}