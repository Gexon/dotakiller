// тут будут описаны все системы специфичные только для растений.
use tinyecs::*;
use time::{PreciseTime, Duration};

use WORLD_SPEED;

use ::ground::components::*;
use ::flora::components::*;


pub struct PlantReproduction;

impl System for PlantReproduction {
    // выбираем сущности содержащие компоненты "FloraClass"
    fn aspect(&self) -> Aspect {
        aspect_all![FloraClass]
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround]]
    }

    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, world: &mut WorldHandle, data: &mut DataList) {
        // перебираем все сущности
        for entity in entities {
            let ground = data.unwrap_entity();
            let wind = ground.get_component::<WindDirection>();

            let mut state = entity.get_component::<FloraState>();

            // если пальма уже жирная, то вбросить семян.
            // пробуем бросить семя через каждые 20 сек
            if state.state >= 10 && state.start.to(PreciseTime::now()) > Duration::seconds(20 * WORLD_SPEED) {
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
                println!("Ветер уносит семена в направлении: {}", wind.direction);
                // поручаем спавнеру, засумонить в наш мир пальму.
                // создаем спавнер
                let entity_spawner = world.entity_manager.create_entity();
                entity_spawner.add_component(SpawnPoint { name: "palm", x: target_x, y: target_y });
                entity_spawner.refresh();
                println!("Пальма размножилась");

                // после удачного засевания
                // фиксируем текущее время
                state.start = PreciseTime::now();
            }
        }
    }
}

pub struct PlantGrowth;

impl System for PlantGrowth {
    // выбираем сущности содержащие компоненты "FloraState"
    fn aspect(&self) -> Aspect {
        aspect_all!(FloraClass)
    }

    // эта функция выполняется во время update столько раз, сколько сущностей содержащих компонент FloraState
    fn process_one(&mut self, entity: &mut Entity) {
        trace!("PLANT GROWTH");
        let mut state = entity.get_component::<FloraState>();

        // инкрементим state, тобиш его рост.
        if state.start.to(PreciseTime::now()) > Duration::seconds(10 * WORLD_SPEED) && state.state < 10 {
            let mut graphic = entity.get_component::<Graphic>();
            state.state += 1;
            graphic.need_replication = true;
            println!("Пальма выросла немного");
            // фиксируем текущее время
            state.start = PreciseTime::now();
        }
    }
}

