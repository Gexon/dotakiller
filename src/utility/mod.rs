// тут будем хранить весь хлам

pub mod logger;
pub mod db_query;
pub mod enums;
pub mod graph_parser;

pub fn init(){
    logger::init();
}