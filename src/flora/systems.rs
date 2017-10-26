// тут будут описаны все системы специфичные только для растений.
// todo Перевести это безобразие на FSM
use tinyecs::*;
use time::{PreciseTime, Duration};

use FLORA_SPEED;

//use ::utility::map::Point;
use ::utility::enums::Direction;
use ::ground::components::*;
use ::flora::components::*;

/// Пальма размножается
pub struct PlantReproductionSystem;

impl System for PlantReproductionSystem {
    // выбираем сущности содержащие компоненты "FloraClass"
    fn aspect(&self) -> Aspect {
        aspect_all!(FloraClass, Reproduction)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround]]
    }

    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, world: &mut WorldHandle, data: &mut DataList) {
        // перебираем все сущности
        for entity in entities {
            let ground = data.unwrap_entity();
            let wind = ground.get_component::<WindDirection>();

            let name = entity.get_component::<Name>();
            let mut state = entity.get_component::<FloraState>();

            // если пальма уже жирная, то вбросить семян.
            // пробуем бросить семя через каждые 10 сек
            if state.reproduction_time.to(PreciseTime::now()) > Duration::seconds(5 * FLORA_SPEED) {
                if name.name != "cactus" {
                    let position = entity.get_component::<Position>();
                    let mut target_x = position.x;
                    let mut target_y = position.y;
                    // определяем координаты выпавшей семки
                    match wind.direction {
                        Direction::North => { if position.x < 140f32 { target_x = position.x + 1f32 } },
                        Direction::NorthWest => {
                            if position.x < 140f32 { target_x = position.x + 1f32 };
                            if position.y > 0f32 { target_y = position.y - 1f32 };
                        },
                        Direction::West => { if position.y > 0f32 { target_y = position.y - 1f32 }; },
                        Direction::WestSouth => {
                            if position.x > 0f32 { target_x = position.x - 1f32 };
                            if position.y > 0f32 { target_y = position.y - 1f32 };
                        },
                        Direction::South => { if position.x > 0f32 { target_x = position.x - 1f32 } },
                        Direction::SouthEast => {
                            if position.x > 0f32 { target_x = position.x - 1f32 };
                            if position.y < 140f32 { target_y = position.y + 1f32 };
                        },
                        Direction::East => { if position.y < 140f32 { target_y = position.y + 1f32 }; },
                        Direction::EastNorth => {
                            if position.x < 140f32 { target_x = position.x + 1f32 };
                            if position.y < 140f32 { target_y = position.y + 1f32 };
                        },
                    }
                    //println!("Ветер уносит семена в направлении: {}", wind.direction);
                    // поручаем спавнеру, засумонить в наш мир пальму.
                    // создаем спавнер
                    let entity_spawner = world.entity_manager.create_entity();
                    entity_spawner.add_component(SpawnFlora { name: "palm", x: target_x, y: target_y });
                    entity_spawner.refresh();
                    //println!("Пальма размножилась");
                }
                // считаем время до умершвления
                state.dead += 1;
                if name.name == "cactus" {
                    if state.dead > 60 {
                        //13
                        // проверяем, не пора ли cactusu в валхаллу
                        entity.add_component(Dead); // пальме пора умереть.
                        entity.remove_component::<Reproduction>(); // выключаем размножение.
                        entity.refresh();
                    }
                } else if state.dead > 6 {
                    //13
                    // проверяем, не пора ли пальме в валхаллу
                    entity.add_component(Dead); // пальме пора умереть.
                    entity.remove_component::<Reproduction>(); // выключаем размножение.
                    entity.refresh();
                }
                // фиксируем текущее время
                state.reproduction_time = PreciseTime::now();
            }
        }
    }
}

/// Пальма растет
pub struct PlantGrowthSystem;

impl System for PlantGrowthSystem {
    // выбираем сущности содержащие компоненты "FloraState"
    fn aspect(&self) -> Aspect {
        aspect_all!(FloraClass, Growth)
    }

    // эта функция выполняется во время update столько раз, сколько сущностей содержащих компонент FloraState
    fn process_one(&mut self, entity: &mut Entity) {
        //trace!("PLANT GROWTH");
        let mut state = entity.get_component::<FloraState>();

        // инкрементим state, тобиш его рост.
        if state.growth_time.to(PreciseTime::now()) > Duration::seconds(FLORA_SPEED) {
            if state.state < 10 {
                state.state += 1;
                entity.add_component(Replication); // требуется репликация.
                entity.refresh();
            }
            // фиксируем таймер для размножения
            if state.state >= 10 {
                entity.add_component(Reproduction); // пальме пора чпокаться.
                entity.remove_component::<Growth>(); // выключаем рост.
                entity.refresh();
            }
            // фиксируем текущее время
            state.growth_time = PreciseTime::now();
        }
    }
}

/// Убиваем пальму
pub struct PlantDeadSystem;

impl System for PlantDeadSystem {
    // выбираем сущности содержащие компоненты FloraClass, Dead
    fn aspect(&self) -> Aspect {
        aspect_all!(FloraClass, Dead)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround]]
    }

    // эта функция выполняется 1 раз.
    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, world: &mut WorldHandle, data: &mut DataList) {
        let ground = data.unwrap_entity();
        let mut world_map = ground.get_component::<WorldMap>();

        // перебираем все сущности
        for entity in entities {
            let name = entity.get_component::<Name>();
            //let id_herb = entity.get_component::<HerbId>(); // удалить, только для лога

            let mut state = entity.get_component::<FloraState>();
            let position = entity.get_component::<Position>();

            // помечаем место как пустое
            let target_point: (i32, i32) = (position.x.trunc() as i32, position.y.trunc() as i32); // Casting
            world_map.flora.remove(&target_point);


            if name.name != "cactus" {
                // поручаем спавнеру, засумонить в наш мир кактус.
                // создаем спавнер
                let entity_spawner = world.entity_manager.create_entity();
                entity_spawner.add_component(SpawnFlora { name: "cactus", x: position.x, y: position.y });
                entity_spawner.refresh();
            }

            state.state = 0; // помечаем состояние как труп.
            entity.remove_component::<Dead>(); //
            entity.add_component(Replication); // требуется репликация.
            entity.add_component(Remove); // требуется убрать с полей.
            entity.refresh();
            //println!("{} {} крякнула", name.name, id_herb.id);
        }
    }
}

/// Убираем с поляны пальму
pub struct PlantRemoveSystem;

impl System for PlantRemoveSystem {
    // выбираем сущности содержащие компоненты "FloraState"
    fn aspect(&self) -> Aspect {
        aspect_all![FloraClass, Remove].except::<Replication>()
    }

    // эта функция выполняется во время update столько раз, сколько сущностей содержащих компонент FloraState
    fn process_one(&mut self, entity: &mut Entity) {
        //entity.remove_component::<Remove>(); // хз, надо ли?
        //entity.remove_component::<FloraClass>(); // если мы удаляем всю сущность целиком однако.
        entity.delete();
    }
}