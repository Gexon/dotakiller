// тут будут описаны все системы специфичные только для растений.
use tinyecs::*;

use ::flora::components::*;


pub struct _PlantReproduction;

impl System for _PlantReproduction {
    // Добавляем систтему "PlantReproduction" к сущностям содержащих компоненты "FloraClass"
    fn aspect(&self) -> Aspect {
        aspect_all!(FloraClass)
    }
}
