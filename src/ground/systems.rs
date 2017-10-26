// общие системы.

use tinyecs::*;
use time::{PreciseTime, Duration};

use GROUND_SPEED;

//use ::utility::map::Point;
//use ::utility::map::Map;
//use ::utility::map::Size;
use ::utility::enums::*;
use ::ground::components::*;
use ::flora::components::*;
use ::monster::components::*;

/// Система создает растения в мире.
pub struct SpawnFloraSystem;

impl System for SpawnFloraSystem {
    // Обрабатываем сущности содержащие компоненты "SpawnPoint", "Position"
    // Аспект - список сущностей, содержащих выбранные компоненты.
    fn aspect(&self) -> Aspect {
        aspect_all!(SpawnFlora)
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
            let spawn_point = entity.get_component::<SpawnFlora>();

            // проверяем свободно ли место спавна.
            let target_point: (i32, i32) = (spawn_point.x.trunc() as i32, spawn_point.y.trunc() as i32); // Casting

            //println!("Пробуем создать сущность: x {}, y {}", target_point.0, target_point.1);
            if !world_map.flora.contains_key(&target_point) {
                world_map.flora.entry(target_point).or_insert_with(last_id.flora_id);
                //world_map.flora.insert(target_point, last_id.flora_id);
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
                    mass: 20,
                });
                entity_object.add_component(HerbId { id: last_id.flora_id });
                entity_object.refresh();
                let id_herb = entity_object.get_component::<HerbId>();
                println!("Создаем сущность {} {}", spawn_point.name.to_string(), id_herb.id);
                last_id.flora_id += 1;
            }

            entity.remove_component::<SpawnFlora>(); // удаляем компонент "Точка спавна/spawn_point"
            entity.delete();
        }
    }
}


/// Система создает монстров в мире.
pub struct SpawnMonsterSystem;

impl System for SpawnMonsterSystem {
    // Обрабатываем сущности содержащие компоненты "SpawnPoint", "Position"
    // Аспект - список сущностей, содержащих выбранные компоненты.
    fn aspect(&self) -> Aspect {
        aspect_all!(SpawnMonster)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround]]
    }

    // обработчик, вызывается при update, process_all - 1 раз вызывается.
    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, world: &mut WorldHandle, data: &mut DataList) {
        let ground = data.unwrap_entity();
        let mut last_id = ground.get_component::<WorldLastId>();

        // перебираем все сущности
        for entity in entities {
            // берем компонент "Точка спавна/spawn_point"
            let spawn_point = entity.get_component::<SpawnMonster>();

            // проверяем свободно ли место спавна.
            //let target_point: Point = Point(spawn_point.x.trunc() as i32, spawn_point.y.trunc() as i32); // Casting

            //println!("Пробуем создать сущность: x {}, y {}", target_point.0, target_point.1);
            // проверяем наличие заданных объектов.
            // создаем объект Монстр.
            let entity_object = world.entity_manager.create_entity();
            entity_object.add_component(Name { name: spawn_point.name.to_string() });
            entity_object.add_component(Position { x: spawn_point.x, y: spawn_point.y });
            entity_object.add_component(MonsterClass {
                growth_time: PreciseTime::now(),
                reproduction_time: PreciseTime::now(),
                behavior_time: PreciseTime::now(),
                bios_time: PreciseTime::now(),
                event_time: PreciseTime::now(),
                selector_time: PreciseTime::now(),
                perception_time: PreciseTime::now()
            });
            entity_object.add_component(Replication); // произошли изменения монстра.
            entity_object.add_component(MonsterState::new());
            entity_object.add_component(MonsterId { id: last_id.monster_id });
            entity_object.add_component(SelectionTree::new());
            entity_object.add_component(BehaviourEvents {
                event: vec![BehaviorEventEnum::NoEvent],
                action: BehaviorActions::Null,

            });
            entity_object.add_component(MonsterAttributes::new());
            entity_object.add_component(MonsterMaps::new());
            entity_object.refresh();
            let monster_id = entity_object.get_component::<MonsterId>();
            println!("Создаем сущность {} {}", spawn_point.name.to_string(), monster_id.id);
            last_id.monster_id += 1;


            entity.remove_component::<SpawnMonster>(); // удаляем компонент "Точка спавна/spawn_point"
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
        if wind.start.to(PreciseTime::now()) > Duration::seconds(8 * GROUND_SPEED) {
            //if wind.direction == 7 { wind.direction = 0 } else { wind.direction += 1; }
            match wind.direction {
                Direction::North => wind.direction = Direction::NorthWest,
                Direction::NorthWest => wind.direction = Direction::West,
                Direction::West => wind.direction = Direction::WestSouth,
                Direction::WestSouth => wind.direction = Direction::South,
                Direction::South => wind.direction = Direction::SouthEast,
                Direction::SouthEast => wind.direction = Direction::East,
                Direction::East => wind.direction = Direction::EastNorth,
                Direction::EastNorth => wind.direction = Direction::North,
            }
            //println!("Ветер меняет направление на {}", wind.direction);
            // фиксируем текущее время
            wind.start = PreciseTime::now();
        }
    }
}

