// тут будут расписаны все компоненты специфичные только для растений.

use tinyecs::*;
use time::PreciseTime;

// метка принадлежности к классу арстений.
pub struct FloraClass;

impl Component for FloraClass {}

pub struct IdHerb {
    pub id: i64,
}

impl Component for IdHerb {}

pub struct FloraState {
    pub state: i32,
    pub start: PreciseTime,
}

impl Component for FloraState {}

