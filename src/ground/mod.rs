// основные компоненты и системы, общие для всех, буду хранить тут.
// так же тут буду размещать компоненты и системы по терраморфингу.
use tinyecs::*;


//use ::ground::components::*;
use ::ground::systems::*;

pub mod components;
pub mod systems;

pub fn init(dk_world: &mut World) {

    // добавляем в мир систему спавна.
    dk_world.set_system(SpawnSystem);
}