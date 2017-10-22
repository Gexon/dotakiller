// тут будем хранить весь хлам

pub mod logger;
pub mod dbqury;
pub mod map;
pub mod enums;
pub mod boxfuture;

pub fn init(){
    logger::init();
}