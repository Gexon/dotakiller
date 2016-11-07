// тут будут расписаны все компоненты специфичные только для растений.

use tinyecs::*;
use time::PreciseTime;

// метка принадлежности к классу арстений.
pub struct FloraClass;

impl Component for FloraClass {}

pub struct _IdHerb {
    id: i32,
}

impl Component for _IdHerb {}

pub struct FloraState {
    pub state: i32,
    pub start: PreciseTime,
}

impl Component for FloraState {}

