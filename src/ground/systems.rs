// общие системы.

use tinyecs::*;
use time::{PreciseTime, Duration};

use WORLD_SPEED;

use ::utility::map::Point;
use ::ground::components::*;
use ::flora::components::*;

/// Система создает объекты в мире.
pub struct SpawnSystem;

impl System for SpawnSystem {
    // Обрабатываем сущности содержащие компоненты "SpawnPoint", "Position"
    // Аспект - список сущностей, содержащих выбранные компоненты.
    fn aspect(&self) -> Aspect {
        aspect_all!(SpawnPoint)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround]]
    }

    // обработчик, вызывается при update
    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, world: &mut WorldHandle, data: &mut DataList) {
        let ground = data.unwrap_entity();
        let mut last_id = ground.get_component::<WorldLastId>();
        let mut world_map = ground.get_component::<WorldMap>();

        // перебираем все сущности
        for entity in entities {
            // берем компонент "Точка спавна/spawn_point"
            let spawn_point = entity.get_component::<SpawnPoint>();

            // проверяем свободно ли место спавна.
            let target_point: Point = Point(spawn_point.x.trunc() as i32, spawn_point.y.trunc() as i32); // Casting

            //println!("Пробуем создать сущность: x {}, y {}", target_point.0, target_point.1);
            if world_map.flora[target_point] == 0 {
                world_map.flora[target_point] = 1;
                world_map.flora[target_point] = 1;

                // проверяем наличие заданных объектов.
                // создаем объект Пальма.
                let entity_object = world.entity_manager.create_entity();
                entity_object.add_component(Name { name: spawn_point.name.to_string() });
                entity_object.add_component(Position { x: spawn_point.x, y: spawn_point.y });
                entity_object.add_component(FloraClass);
                entity_object.add_component(Growth);
                entity_object.add_component(Replication); // требуется репликация.
                entity_object.add_component(FloraState {
                    state: 1,
                    growth_time: PreciseTime::now(),
                    reproduction_time: PreciseTime::now(),
                    dead: 0,
                });
                entity_object.add_component(IdHerb { id: last_id.flora_id });
                entity_object.refresh();
                let id_herb = entity_object.get_component::<IdHerb>();
                println!("Создаем сущность {} {}", spawn_point.name.to_string(), id_herb.id);
                last_id.flora_id += 1;
            }

            entity.remove_component::<SpawnPoint>(); // удаляем компонент "Точка спавна/spawn_point"
            entity.delete();
        }
    }
}

/// система меняет направление вветра
pub struct WindDirectionSystem;

impl System for WindDirectionSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(ClassGround)
    }

    fn process_one(&mut self, entity: &mut Entity) {
        //trace!("WIND Direction");
        let mut wind = entity.get_component::<WindDirection>();

        // меняем ветер
        if wind.start.to(PreciseTime::now()) > Duration::seconds(8 * WORLD_SPEED) {
            if wind.direction == 7 { wind.direction = 0 } else { wind.direction += 1; }
            //println!("Ветер меняет направление на {}", wind.direction);
            // фиксируем текущее время
            wind.start = PreciseTime::now();
        }
    }
}

