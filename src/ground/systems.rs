// общие системы.

use tinyecs::*;
use time::{PreciseTime, Duration};

use GROUND_SPEED;

use ::utility::map::Point;
use ::utility::map::Map;
use ::utility::map::Size;
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
            let target_point: Point = Point(spawn_point.x.trunc() as i32, spawn_point.y.trunc() as i32); // Casting

            //println!("Пробуем создать сущность: x {}, y {}", target_point.0, target_point.1);
            if world_map.flora[target_point] == 0 {
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


/// система событий для растений
// события поедания растений монстрами.
pub struct FloraEventSystem;

impl System for FloraEventSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(FloraClass)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround]]
    }

    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, _world: &mut WorldHandle, data: &mut DataList) {
        let ground = data.unwrap_entity();
        let mut event_to_flora = ground.get_component::<EventsTo>();
        // извлекаем события из вектора событий от монстров
        let mut event_vec = &mut event_to_flora.event_eat_flora;
        if !event_vec.is_empty() {
            {
                let event = event_vec.last_mut().unwrap();

                // перебираем все сущности
                for entity in entities {
                    let mut flora_state = entity.get_component::<FloraState>();
                    let flora_position = entity.get_component::<Position>();
                    //let flora_id = entity.get_component::<HerbId>();
                    //println!("event.pop растение {}", flora_id.id);
                    //if flora_position.x == event.x && flora_position.y == event.y {
                    if (flora_position.x - event.x).abs() < ::std::f32::EPSILON &&
                        (flora_position.y - event.y).abs() < ::std::f32::EPSILON {
                        if flora_state.mass > event.value {
                            flora_state.mass -= event.value;
                        } else { self.kill_flora(entity, &mut flora_state.mass) }
                        //println!("Растение:{}, масса:{}", flora_id.id, flora_state.mass);
                    }
                }
            }
            // убираем из очереди событйи, для предотвращения зацикливания при отсутствии растения на месте.
            event_vec.pop().unwrap();
        }
    }
}

impl FloraEventSystem {
    fn kill_flora(&self, entity: &Entity, mass: &mut i32) {
        *mass = 0;
        if entity.has_component::<Growth>() { entity.remove_component::<Growth>(); }
        if entity.has_component::<Reproduction>() { entity.remove_component::<Reproduction>(); }
        entity.add_component(Dead);
        entity.refresh();
    }
}


/// система событий для монстров
// обработка только запросов на вступлении в группу
pub struct MonsterRequestJoinGroupSystem;

impl System for MonsterRequestJoinGroupSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround]]
    }

    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, _world: &mut WorldHandle, data: &mut DataList) {
        let ground = data.unwrap_entity();
        let mut events_to = ground.get_component::<EventsTo>();

        // События для вожака о вступлении в группу.(от кого, кому)
        if !events_to.request_join_group.is_empty() {
            let sender = events_to.request_join_group.last_mut().unwrap().0; // тот далекий крабик
            let recipient = events_to.request_join_group.last_mut().unwrap().1; // этот, краб, он принимает приглос.
            // перебираем все сущности
            for entity in entities {
                let mut monster_attr = entity.get_component::<MonsterAttributes>();
                let monster_id = entity.get_component::<MonsterId>();
                println!("request_join_group.pop монстр {}", monster_id.id);
                // сравнить id отправителя, чтоб самого себя не приглашануть
                if sender != monster_id.id
                    // проверяем не в группе ли крабина
                    && !monster_attr.in_group {
                    self.add_group(&mut monster_attr, sender, recipient, &mut events_to.answer_accepting_group);
                }
            }
            // убираем из очереди событйи, для предотвращения зацикливания при отсутствии монстра на месте.
            events_to.request_join_group.pop().unwrap();
        }
    }
}

// всякие функции для лучшего чтения кода
impl MonsterRequestJoinGroupSystem {
    // зачисление в группу отдельного члена стаи
    fn add_group(&self,
                 monster_attr: &mut MonsterAttributes,
                 sender: i32,
                 recipient: i32,
                 answer_vec: &mut Vec<(i32, i32)>,
    ) {
        // метим краба что он в группе.
        monster_attr.in_group = true;
        monster_attr.id_lead = sender;
        monster_attr.lead = false;
        // шлем лидеру сообщение что он стал вожаком и у него есть стая!
        answer_vec.push((recipient, sender));
    }
}


/// система событий для монстров
// обработка только подтверждения о вступлении в группу
pub struct MonsterAcceptingGroupSystem;

impl System for MonsterAcceptingGroupSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround]]
    }

    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, _world: &mut WorldHandle, data: &mut DataList) {
        let ground = data.unwrap_entity();
        let mut events_to = ground.get_component::<EventsTo>();
        // Ответ от члена стаи о принятии приглоса от вожака для вожака.(от кого, кому)
        if !events_to.answer_accepting_group.is_empty() {
            let sender = events_to.answer_accepting_group.last_mut().unwrap().0; // тот далекий крабик
            let recipient = events_to.answer_accepting_group.last_mut().unwrap().1; // этот, краб, он принимает приглос.
            // перебираем все сущности
            for entity in entities {
                let mut monster_attr = entity.get_component::<MonsterAttributes>();
                let monster_id = entity.get_component::<MonsterId>();
                println!("answer_accepting_group.pop монстр {}", monster_id.id);
                // сравнить id отправителя, чтоб самого себя не того
                if sender != monster_id.id {
                    self.accepting_group(&mut monster_attr, sender, recipient, );
                }
            }
            // убираем из очереди событйи, для предотвращения зацикливания при отсутствии монстра на месте.
            events_to.answer_accepting_group.pop().unwrap();
        }
    }
}

// всякие функции для лучшего чтения кода
impl MonsterAcceptingGroupSystem {
    // вожак вносит члена стаи в группу
    fn accepting_group(&self,
                       monster_attr: &mut MonsterAttributes,
                       sender: i32,
                       recipient: i32,
    ) {
        // делаем пометки о новом члене стаи
        monster_attr.in_group = true;
        monster_attr.id_lead = recipient;
        monster_attr.lead = true;
        monster_attr.group_members.push(sender);
    }
}


/// Обновление карты монстров
pub struct MonsterMapSystem;

impl System for MonsterMapSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround]]
    }

    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, _world: &mut WorldHandle, data: &mut DataList) {
        let ground = data.unwrap_entity();
        let mut world_map = ground.get_component::<WorldMap>();
        // Удаляем содержимое карты.
        world_map.monster = Map::new(
            Size(140, 140),
            Vec::new(),
            Vec::new(),
        );

        // перебираем все сущности
        for entity in entities {
            let position = entity.get_component::<Position>();
            let monster_id = entity.get_component::<MonsterId>();
            // оперделяем место
            let target_point: Point = Point(position.x as i32, position.y as i32);
            // втыкаем ее в карту
            let vec = &mut world_map.monster[target_point];
            vec.push(monster_id.id);
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

