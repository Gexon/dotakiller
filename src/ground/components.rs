// общие копоненты для всех сущностей.

use tinyecs::*;

pub struct Name {
    pub name: String
}

impl Component for Name {}

// репликация клиенту изменений.
pub struct Graphic{
    pub need_replication: bool,
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
    pub count: i32
}

impl Component for SpawnPoint {}