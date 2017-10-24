// тут будем обрабатывать все события от растений и монстров

use tinyecs::*;

//use ::utility::map::Point;
//use ::utility::map::Map;
//use ::utility::map::Size;
use ::utility::enums::*;
use ::ground::components::*;
use ::flora::components::*;
use ::monster::components::*;


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
        let mut events_to = ground.get_component::<EventsTo>();
        // извлекаем вектор с событиями из компоненты EventsTo
        let events_vec = &mut events_to.events_eat_flora;
        if !events_vec.is_empty() {
            {
                let event: &mut EventEatFlora = events_vec.last_mut().unwrap();

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
                        break; // должен прервать for in
                    }
                }
            }
            // убираем из очереди событйи, для предотвращения зацикливания при отсутствии растения на месте.
            events_vec.pop().unwrap();
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
// обработка входящих событий
pub struct MonsterEventSystem;

impl System for MonsterEventSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround]]
    }

    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, _world: &mut WorldHandle, data: &mut DataList) {
        let ground = data.unwrap_entity(); // общая сущность, хранит в себе очередь событий.
        let mut events_to = ground.get_component::<EventsTo>(); // очередь событий, всех, и флора и монстры.
        // извлекаем вектор с событиями из компоненты EventsTo
        let mut events_vec: &mut Vec<EventGroupMonster> = &mut events_to.events_group_monster;
        // чекаем события из очереди событий групп монстра.
        if !events_vec.is_empty() {
            {
                // получаем ссылку на последнее событие в очереди
                let event: &mut EventGroupMonster = events_vec.last_mut().unwrap();

                // работа с запросами RequestJoinGroup
                // событие для ВОЖДЯ, прошение вступить в группу.
                if event.event_type == EventTypeMonster::RequestJoinGroup {
                    let sender = event.id_sender; // отправитель запроса
                    let recipient = event.id_receiver; // этот краб - ВОЖДЬ, он принимает приглос.
                    // вынимаем вождя
                    for entity in entities {
                        let monster_id = entity.get_component::<MonsterId>();
                        if monster_id.id == recipient {
                            println!("RequestJoinGroup.pop монстр-ВОЖДЬ {}", monster_id.id);
                            let mut monster_attr = entity.get_component::<MonsterAttributes>(); // атрибуты ВОЖДЯ
                            self.add_to_group(&mut monster_attr, sender, recipient, &mut events_vec);
                            break;
                        }
                    }
                }
            }

//            {
//                // работа с событием AnswerAcceptingGroup
//                // событие для просителя, меня приняли в группу.
//                if event.event_type == EventTypeMonster::AnswerAcceptingGroup {
//                    let sender = event.id_sender; // отправитель запроса ВОЖДЬ
//                    let recipient = event.id_receiver; // этот краб, новичек
//                    // вынимаем получателя-новичка
//                    for entity in entities {
//                        let monster_id = entity.get_component::<MonsterId>();
//                        if monster_id.id == recipient {
//                            println!("AnswerAcceptingGroup.pop монстр-новичек {}", monster_id.id);
//                            let mut monster_attr = entity.get_component::<MonsterAttributes>();
//                            self.accepting_group(&mut monster_attr, sender);
//                            break;
//                        }
//                    }
//                }
//            }

            // убираем из очереди событйи, для предотвращения зацикливания при отсутствии монстра на месте.
            events_vec.pop().unwrap();
        }
    }
}


impl MonsterEventSystem {
    // вожак вносит члена стаи в группу
    fn add_to_group(&self,
                    monster_attr: &mut MonsterAttributes, // атрибуты ВОЖДЯ
                    sender: i32,
                    recipient: i32,
                    events_vec: &mut Vec<EventGroupMonster>,
    ) {
        // делаем пометки о новом члене стаи
        monster_attr.in_group = true;
        monster_attr.id_lead = recipient;
        monster_attr.lead = true;
        monster_attr.group_members.push(sender);


        // шлем просителю сообщение что он стал членом стаи.
        events_vec.push(EventGroupMonster {
            event_type: EventTypeMonster::AnswerAcceptingGroup,
            id_sender: recipient,
            id_receiver: sender,
        });
    }


    // принимаем членство в группе.
    fn accepting_group(&self,
                       monster_attr: &mut MonsterAttributes,
                       sender: i32,
    ) {
        // метим краба что он в группе.
        monster_attr.in_group = true;
        monster_attr.id_lead = sender;
        monster_attr.lead = false;
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

    fn process_all(&mut self, _entities: &mut Vec<&mut Entity>, _world: &mut WorldHandle, _data: &mut DataList) {
        //let ground = data.unwrap_entity();
        //let mut world_map = ground.get_component::<WorldMap>();
        // Удаляем содержимое карты.
        //        world_map.monster = Map::new(
        //            Size(140, 140),
        //            Vec::new(),
        //            Vec::new(),
        //        );

        // перебираем все сущности
        //        for entity in entities {
        //            let position = entity.get_component::<Position>();
        //            let monster_id = entity.get_component::<MonsterId>();
        //            // оперделяем место
        //            let target_point: Point = Point(position.x as i32, position.y as i32);
        //            // втыкаем ее в карту
        //            let vec = &mut world_map.monster[target_point];
        //            vec.push(monster_id.id);
        //        }
    }
}