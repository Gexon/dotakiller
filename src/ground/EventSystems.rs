// тут будем обрабатывать все события от растений и монстров

use tinyecs::*;
use time::{PreciseTime, Duration};

use GROUND_SPEED;

use ::utility::map::Point;
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
        // извлекаем события из вектора событий от монстров
        let event_vec = &mut events_to.events_eat_flora;
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
                        break; // должен прервать for in
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

        // чекаем события из очереди событий групп монстра.
        if !events_to.events_group_monster.is_empty() {
            // работа с запросами RequestJoinGroup
            {
                // получаем ссылку на последнее событие в очереди
                let event: EventGroupMonster = events_to.events_group_monster.last_mut().unwrap();
                // События для вожака о вступлении в группу.(МОНСТР -> ВОЖДЮ)
                if event.event == EventTypeMonster.RequestJoinGroup {
                    let sender = event.id_sender; // отправитель запроса
                    let recipient = event.id_receiver; // этот краб - ВОЖДЬ, он принимает приглос.
                    // перебираем все сущности
                    for entity in entities {
                        let monster_id = entity.get_component::<MonsterId>();
                        if monster_id == recipient {
                            println!("RequestJoinGroup.pop монстр sender {}", monster_id.id);
                            // сравнить id отправителя, чтоб самого себя не приглашануть
                            let mut monster_attr = entity.get_component::<MonsterAttributes>();
                            if sender != monster_id.id
                                // проверяем не в группе ли крабина
                                && !monster_attr.in_group {
                                self.add_group(&mut monster_attr, sender, recipient, &mut events_to);
                            }
                        }
                        break;
                    }
                }

                // тут обрабатываем остальные типы событий
            }

            // убираем из очереди событйи, для предотвращения зацикливания при отсутствии монстра на месте.
            events_to.events_group_monster.pop().unwrap();
        }
    }
}

// всякие функции для лучшего чтения кода
impl MonsterEventSystem {
    // зачисление в группу отдельного члена стаи
    fn add_group(&self,
                 monster_attr: &mut MonsterAttributes,
                 sender: i32,
                 recipient: i32,
                 answer_vec: &mut Vec<(i32, i32)>,
    ) {
        // метим краба что он в группе.
        monster_attr.in_group = true;
        monster_attr.id_lead = recipient;
        monster_attr.lead = false;
        // шлем лидеру сообщение что он стал вожаком и у него есть стая!
        // AnswerAcceptingGroup
        answer_vec.push(EventGroupMonster {
            event: EventTypeMonster::AnswerAcceptingGroup,
            id_sender: recipient,
            id_receiver: sender,
        });
    }

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


/// система событий для монстров
// обработка только запросов на вступление в группу
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
                    self.accepting_group(&mut monster_attr, sender, recipient);
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