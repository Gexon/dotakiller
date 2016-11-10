// тут будут описаны все системы специфичные только для растений.
use tinyecs::*;
use time::{PreciseTime, Duration};

use WORLD_SPEED;

use ::utility::map::Point;
use ::ground::components::*;
use ::flora::components::*;


pub struct PlantReproductionSystem;

/// Пальма размножается
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
            if state.reproduction_time.to(PreciseTime::now()) > Duration::seconds(10 * WORLD_SPEED) && name.name != "cactus" {
                if name.name == "cactus" {
                    state.dead += 1;
                    if state.dead > 6 {
                        // проверяем, не пора ли пальме в валхаллу
                        entity.add_component(Dead); // пальме пора умереть.
                        entity.remove_component::<Reproduction>(); // выключаем рост.
                        entity.refresh();
                    }
                } else {
                    let position = entity.get_component::<Position>();
                    let mut target_x = position.x;
                    let mut target_y = position.y;
                    // определяем координаты выпавшей семки
                    match wind.direction {
                        0 => { if position.x < 140f32 { target_x = position.x + 1f32 } },
                        1 => {
                            if position.x < 140f32 { target_x = position.x + 1f32 };
                            if position.y > 0f32 { target_y = position.y - 1f32 };
                        },
                        2 => { if position.y > 0f32 { target_y = position.y - 1f32 }; },
                        3 => {
                            if position.x > 0f32 { target_x = position.x - 1f32 };
                            if position.y > 0f32 { target_y = position.y - 1f32 };
                        },
                        4 => { if position.x > 0f32 { target_x = position.x - 1f32 } },
                        5 => {
                            if position.x > 0f32 { target_x = position.x - 1f32 };
                            if position.y < 140f32 { target_y = position.y + 1f32 };
                        },
                        6 => { if position.y < 140f32 { target_y = position.y + 1f32 }; },
                        7 => {
                            if position.x < 140f32 { target_x = position.x + 1f32 };
                            if position.y < 140f32 { target_y = position.y + 1f32 };
                        },
                        _ => println!("странное направление ветра, вы не находите?"),
                    }
                    //println!("Ветер уносит семена в направлении: {}", wind.direction);
                    // поручаем спавнеру, засумонить в наш мир пальму.
                    // создаем спавнер
                    let entity_spawner = world.entity_manager.create_entity();
                    entity_spawner.add_component(SpawnPoint { name: "palm", x: target_x, y: target_y });
                    entity_spawner.refresh();
                    //println!("Пальма размножилась");

                    // после удачного засевания
                    // фиксируем текущее время
                    state.reproduction_time = PreciseTime::now();
                    // считаем время до умершвления
                    state.dead += 1;
                    if state.dead > 12 {
                        // проверяем, не пора ли пальме в валхаллу
                        entity.add_component(Dead); // пальме пора умереть.
                        entity.remove_component::<Reproduction>(); // выключаем рост.
                        entity.refresh();
                    }
                }
            }
        }
    }
}

pub struct PlantGrowthSystem;

/// Пальма растет
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
        if state.growth_time.to(PreciseTime::now()) > Duration::seconds(2 * WORLD_SPEED) {
            let mut graphic = entity.get_component::<Graphic>();
            if state.state < 10 {
                state.state += 1;
                graphic.need_replication = true;
            }
            //println!("Пальма выросла немного");
            // фиксируем текущее время
            state.growth_time = PreciseTime::now();
            // фиксируем таймер для отрупнения
            if state.state == 10 {
                entity.add_component(Reproduction); // пальме пора чпокаться.
                entity.remove_component::<Growth>(); // выключаем рост.
                entity.refresh();
            }
        }
    }
}

pub struct PlantDeadSystem;

/// Убиваем пальму
impl System for PlantDeadSystem {
    // выбираем сущности содержащие компоненты "FloraState"
    fn aspect(&self) -> Aspect {
        aspect_all!(FloraClass, Dead)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround]]
    }

    // эта функция выполняется во время update столько раз, сколько сущностей содержащих компонент FloraState
    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, world: &mut WorldHandle, data: &mut DataList) {
        let ground = data.unwrap_entity();
        let mut world_map = ground.get_component::<WorldMap>();

        // перебираем все сущности
        for entity in entities {
            let name = entity.get_component::<Name>();
            let mut state = entity.get_component::<FloraState>();
            let mut graphic = entity.get_component::<Graphic>();
            if state.state != 0 {
                state.state = 0;
                graphic.need_replication = true;
                let id_herb = entity.get_component::<IdHerb>();
                println!("{} {} крякнула", name.name, id_herb.id);

                let position = entity.get_component::<Position>();
                // помечаем место как пустое
                let target_point: Point = Point(position.x.trunc() as i32, position.y.trunc() as i32); // Casting
                world_map.flora[target_point] = 0;

                if name.name != "cactus" {
                    // поручаем спавнеру, засумонить в наш мир кактус.
                    // создаем спавнер
                    let entity_spawner = world.entity_manager.create_entity();
                    entity_spawner.add_component(SpawnPoint { name: "cactus", x: position.x, y: position.y });
                    //entity_spawner.refresh();
                }
            }
        }
    }
}