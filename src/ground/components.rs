// общие копоненты для всех сущностей.

use tinyecs::*;
use time::PreciseTime;

use ::utility::map::Map;

pub struct Name {
    pub name: String
}

impl Component for Name {}

pub struct ClassGround;

impl Component for ClassGround {}

// храним последние id
pub struct WorldLastId {
    pub flora_id: i64,
}

impl Component for WorldLastId {}
// тут будем хранить все объекты на карте.
pub struct WorldMap {
    pub flora: Map<u8>,
}

impl Component for WorldMap {}

// куда дует ветер. 0 - это типа север(+Х)
pub struct WindDirection {
    pub direction: u8,
    pub start: PreciseTime,
}

impl Component for WindDirection {}

// репликация клиенту изменений.
pub struct Graphic {
    //pub need_replication: bool,
}

impl Component for Graphic {
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

pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {}

pub struct SpawnPoint {
    pub name: &'static str,
    pub x: f32,
    pub y: f32,
}

impl Component for SpawnPoint {}

/// Все что должно расти - должно расти.
pub struct Growth;

impl Component for Growth {}

/// Все что должно размножаться.
pub struct Reproduction;

impl Component for Reproduction {}

/// Все что должно умереть.
pub struct Dead;

impl Component for Dead {}

/// Репликация
pub struct Replication;

impl Component for Replication {}