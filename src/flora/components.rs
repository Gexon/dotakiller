// тут будут расписаны все компоненты специфичные только для растений.

use tinyecs::*;

// метка принадлежности к классу арстений.
pub struct FloraClass;

impl Component for FloraClass {}

pub struct _IdHerb{
    id: i32,
}

impl Component for _IdHerb{}



