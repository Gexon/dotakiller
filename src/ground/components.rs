// общие копоненты для всех сущностей.

use tinyecs::*;
use time::PreciseTime;
use std::collections::HashMap;

//use ::utility::map::Map;
use ::utility::enums::Direction;
use ::utility::enums::EventEatFlora;
use ::utility::enums::EventGroupMonster;


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


/// Сообщения от монстров для пальм
pub struct EventsTo {
    // События на поедание пальмы.
    pub events_eat_flora: Vec<EventEatFlora>,
    // События для взаимодействия при работе групп
    pub events_group_monster: Vec<EventGroupMonster>,
}

impl Component for EventsTo {}

impl EventsTo {
    pub fn new() -> EventsTo {
        EventsTo {
            events_eat_flora: Vec::new(),
            events_group_monster: Vec::new(),
        }
    }
}


// храним последние id
pub struct WorldLastId {
    pub flora_id: i32,
    pub monster_id: i32,
}

impl Component for WorldLastId {}


// тут будем хранить все статичные объекты на карте.
pub struct WorldMap {
    pub flora: HashMap<(i32, i32), i32>, // (координата х, координата у), id
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