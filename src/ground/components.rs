// общие копоненты для всех сущностей.

use tinyecs::*;
use time::PreciseTime;

use ::utility::map::Map;
use ::utility::enums::Direction;


pub struct ClassGround;

impl Component for ClassGround {}


/// Координаты на полигоне.
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {}


/// имя
pub struct Name {
    pub name: String
}

impl Component for Name {}


// храним последние id
pub struct WorldLastId {
    pub flora_id: i64,
    pub monster_id: i64,
}

impl Component for WorldLastId {}


// тут будем хранить все объекты на карте.
pub struct WorldMap {
    pub flora: Map<u8>,
    pub monster: Map<u8>,
}

impl Component for WorldMap {}

// куда дует ветер. 0 - это типа север(+Х)
pub struct WindDirection {
    pub direction: Direction,
    pub start: PreciseTime,
}

impl Component for WindDirection {}

// репликация клиенту изменений.
pub struct _Graphic {
    //pub need_replication: bool,
}

impl Component for _Graphic {
    // представление в графической подсистеме
    // репликация клиенту изменений.
}

// сложная хрень.
pub struct _Physic;

impl Component for _Physic {
    // представление в физической подсистеме
    // обработка входящих событий, обработка внутренних процессов, изменение состояний.
    // биология тут же будет
}


/// Для спавна травы
pub struct SpawnFlora {
    pub name: &'static str,
    pub x: f32,
    pub y: f32,
}

impl Component for SpawnFlora {}


/// Для спавна монстров
pub struct SpawnMonster {
    pub name: &'static str,
    pub x: f32,
    pub y: f32,
}

impl Component for SpawnMonster {}


/// Все что должно расти - должно расти.
pub struct Growth;

impl Component for Growth {}

/// Все что должно размножаться.
pub struct Reproduction;

impl Component for Reproduction {}

/// Помечаем как мертвый и реплицируем.
pub struct Dead;

impl Component for Dead {}

/// Удаляем из поляны.
pub struct Remove;

impl Component for Remove {}

/// Репликация
pub struct Replication;

impl Component for Replication {}