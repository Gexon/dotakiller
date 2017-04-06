// тут будут расписаны все компоненты специфичные только для растений.


use tinyecs::*;
use time::PreciseTime;


/// метка принадлежности к классу арстений.
pub struct FloraClass;

impl Component for FloraClass {}


/// уникальный номер растения
pub struct HerbId {
    pub id: i64,
}

impl Component for HerbId {}


/// информация о состоянии растения
pub struct FloraState {
    pub state: i32,
    pub growth_time: PreciseTime,
    pub reproduction_time: PreciseTime,
    pub dead: i32,
    pub mass: i32,
}

impl Component for FloraState {}

