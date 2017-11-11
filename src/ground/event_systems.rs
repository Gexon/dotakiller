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
                        //println!("Монстр сожрал растение:{}, масса:{}", flora_id.id, flora_state.mass);
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
// https://play.rust-lang.org/?gist=a1ca5d577fd0343a3dedf55ddf4c9968&version=stable
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
        // оставляю общую проверку на присутствие событий в очереди, преждевременная оптимизация
        if !events_vec.is_empty() {
            // временная переменная для характеристик монстра
            let mut temp_event_accept: EventGroupMonster = EventGroupMonster {
                event_type: EventTypeMonster::None,
                id_sender: 0,
                id_receiver: 0,
            };
            // временная переменная для характеристик монстра
            let mut temp_event_leave: EventGroupMonster = EventGroupMonster {
                event_type: EventTypeMonster::None,
                id_sender: 0,
                id_receiver: 0,
            };
            // временная переменная для характеристик монстра
            let mut vec_event_leave_lead = vec![];

            // обработка запроса на вхождение в группу
            if !events_vec.is_empty() {
                let mut in_entry = false;
                let mut accepted = false;
                {
                    // получаем ссылку на последнее событие в очереди
                    let event: &mut EventGroupMonster = events_vec.last_mut().unwrap();
                    // работа с запросами RequestJoinGroup
                    // событие для ВОЖДЯ, прошение вступить в группу.
                    if event.event_type == EventTypeMonster::RequestJoinGroup {
                        in_entry = true;
                        let sender = event.id_sender; // отправитель запроса
                        let recipient = event.id_receiver; // этот краб - ВОЖДЬ, он принимает приглос.
                        // вынимаем ВОЖДЯ
                        for entity in &*entities {
                            let monster_id = entity.get_component::<MonsterId>();
                            if monster_id.id == recipient {
                                println!("Пришел запрос от монстра {} на вступление в твою группу, уважаемый  ВОЖДЬ {}", sender, monster_id.id);
                                let mut monster_attr = entity.get_component::<MonsterAttributes>(); // атрибуты ВОЖДЯ
                                // а настоящий ли ВОЖДЬ или может ли монстр стать ВОЖДЕМ.
                                if (monster_attr.in_group && monster_attr.lead) || !monster_attr.in_group {
                                    temp_event_accept = self.add_to_group(&mut monster_attr, sender, recipient); // делаем пометки о новом члене
                                    accepted = true;
                                    break;
                                }
                            }
                        }
                    }
                }
                if in_entry {
                    // убираем из очереди событйи, для предотвращения зацикливания при отсутствии монстра на месте.
                    events_vec.pop().unwrap();
                    if accepted {
                        self.send_accept_event(temp_event_accept, &mut events_vec);
                    } else { println!("В запросе отказано"); }
                }
            }


            // погнали второй трай, тут будем принимать нашим монстром-просителем ответ от ВОЖДЯ
            if !events_vec.is_empty() {
                let mut in_entry = false;
                {
                    // получаем ссылку на последнее событие в очереди
                    let event: &mut EventGroupMonster = events_vec.last_mut().unwrap();
                    // работа с событием AnswerAcceptingGroup
                    // событие для просителя, меня приняли в группу.
                    if event.event_type == EventTypeMonster::AnswerAcceptingGroup {
                        in_entry = true;
                        let sender = event.id_sender; // отправитель запроса ВОЖДЬ
                        let recipient = event.id_receiver; // этот краб, новичек
                        // вынимаем получателя-новичка
                        for entity in &*entities {
                            let monster_id = entity.get_component::<MonsterId>();
                            if monster_id.id == recipient {
                                println!("Пришел ответ от ВОЖДЯ {} на вступление монстра {} в группу", sender, monster_id.id);
                                let mut monster_attr = entity.get_component::<MonsterAttributes>();
                                self.accepting_group(&mut monster_attr, sender);
                                break;
                            }
                        }
                    }
                }
                if in_entry {
                    // убираем из очереди событйи, для предотвращения зацикливания при отсутствии монстра на месте.
                    events_vec.pop().unwrap();
                }
            }


            // сообщение о выходе члена из стаи
            if !events_vec.is_empty() {
                let mut in_entry = false;
                {
                    // получаем ссылку на последнее событие в очереди
                    let event: &mut EventGroupMonster = events_vec.last_mut().unwrap();
                    // работа с запросами MessageLeaveGroup
                    // событие для ВОЖДЯ, извещение о выходе из этой унылой группы
                    if event.event_type == EventTypeMonster::MessageLeaveGroup {
                        in_entry = true;
                        let sender = event.id_sender; // отправитель запроса
                        let recipient = event.id_receiver; // этот краб - ВОЖДЬ, он принимает извещение.
                        // вынимаем ВОЖДЯ
                        for entity in &*entities {
                            let monster_id = entity.get_component::<MonsterId>();
                            if monster_id.id == recipient {
                                println!("Пришело смс. Монстр {} ливает из группы, хладнокровный ВОЖДЬ", monster_id.id);
                                let mut monster_attr = entity.get_component::<MonsterAttributes>(); // атрибуты ВОЖДЯ
                                temp_event_leave = self.leave_in_group(&mut monster_attr, sender, recipient); // выпиливаем монстра с группы
                                break;
                            }
                        }
                    }
                }
                if in_entry {
                    // убираем из очереди событйи, для предотвращения зацикливания при отсутствии монстра на месте.
                    events_vec.pop().unwrap();
                    self.send_leave_event(temp_event_leave, &mut events_vec);
                }
            }


            // ответ члену о исключении из стаи
            if !events_vec.is_empty() {
                let mut in_entry = false;
                {
                    // получаем ссылку на последнее событие в очереди
                    let event: &mut EventGroupMonster = events_vec.last_mut().unwrap();
                    // работа с запросами AnswerLeavingGroup
                    // событие для члена, извещение о исключении из группы
                    if event.event_type == EventTypeMonster::AnswerLeavingGroup {
                        in_entry = true;
                        let sender = event.id_sender; // отправитель ВОЖДЬ
                        let recipient = event.id_receiver; // этот краб - член, он принимает извещение.
                        // вынимаем члена
                        for entity in &*entities {
                            let monster_id = entity.get_component::<MonsterId>();
                            if monster_id.id == recipient {
                                println!("Пришело смс от ВОЖДЯ {}: монстр {} выпилен из группы", sender, monster_id.id);
                                let mut monster_attr = entity.get_component::<MonsterAttributes>(); // атрибуты ВОЖДЯ
                                self.leaving_group(&mut monster_attr); // выпиливаем монстра с группы
                                break;
                            }
                        }
                    }
                }
                if in_entry {
                    // убираем из очереди событйи, для предотвращения зацикливания при отсутствии монстра на месте.
                    events_vec.pop().unwrap();
                }
            }

            // ВОЖДЬ покидает стаю
            if !events_vec.is_empty() {
                let mut in_entry = false;
                {
                    // получаем ссылку на последнее событие в очереди
                    let event: &mut EventGroupMonster = events_vec.last_mut().unwrap();

                    // работа с запросами LeadLeaveGroup
                    // событие для ВСЕХ, о выходе ВОЖДЯ из группы
                    if event.event_type == EventTypeMonster::LeadLeaveGroup {
                        in_entry = true;
                        let sender = event.id_sender; // отправитель ВОЖДЬ
                        // вынимаем ВОЖДЯ
                        for entity in &*entities {
                            let monster_id = entity.get_component::<MonsterId>();
                            if monster_id.id == sender {
                                println!("ВОЖДЬ {} покинул группу", sender);
                                let mut monster_attr = entity.get_component::<MonsterAttributes>(); // атрибуты ВОЖДЯ
                                vec_event_leave_lead = self.leave_lead_in_group(&mut monster_attr, sender); // выпиливаем монстра с группы
                                break;
                            }
                        }
                    }
                }
                if in_entry {
                    // убираем из очереди событйи, для предотвращения зацикливания при отсутствии монстра на месте.
                    events_vec.pop().unwrap();
                    self.send_leave_lead_event(vec_event_leave_lead, &mut events_vec);
                }
            }
        }
    }
}


impl MonsterEventSystem {
    // ВОЖДЬ вносит члена стаи в группу
    fn add_to_group(&self,
                    monster_attr: &mut MonsterAttributes,
                    sender: i32,
                    recipient: i32,
    ) -> EventGroupMonster
    {
        // делаем пометки о новом члене стаи
        monster_attr.in_group = true;
        monster_attr.id_lead = recipient;
        monster_attr.lead = true;
        monster_attr.group_members.push(sender);
        // готовим результат
        println!("Создаем событие: AnswerAcceptingGroup от ВОЖДЯ {} к монстру просителю {}", recipient, sender);
        EventGroupMonster {
            event_type: EventTypeMonster::AnswerAcceptingGroup,
            // меняем местами, т.к. теперь ВОЖДЬ шлет смску
            id_sender: recipient,
            id_receiver: sender,
        }
    }

    // ВОЖДЬ шлет смс монстру, что все ОК
    fn send_accept_event(&self,
                         temp_event: EventGroupMonster,
                         events_vec: &mut Vec<EventGroupMonster>,
    ) {
        // шлем просителю сообщение что он стал членом стаи
        if temp_event.event_type != EventTypeMonster::None {
            events_vec.push(temp_event);
            println!("Запрос от монстра на вступление в группу удовлетворен");
        } else { println!("send_accept_event -> Что-то пошло не так, шлем пустые события!"); };
    }

    // принимаем членство в группе.
    fn accepting_group(&self,
                       monster_attr: &mut MonsterAttributes,
                       sender: i32,
    ) {
        // метим краба что он в группе.
        monster_attr.in_group = true;
        monster_attr.id_lead = sender; // id ВОЖДЯ
        monster_attr.lead = false;
    }

    // ВОЖДЬ выписывает члена стаи из группы
    fn leave_in_group(&self,
                      monster_attr: &mut MonsterAttributes,
                      sender: i32,
                      recipient: i32,
    ) -> EventGroupMonster
    {
        // выпиливаем отщепенца со списка группы
        monster_attr.group_members.retain(|&item| item != sender);
        // если список членов пуст, ВОЖДЬ уже не ВОЖДЬ =(
        if monster_attr.group_members.is_empty() {
            monster_attr.in_group = false;
            monster_attr.id_lead = -1;
            monster_attr.lead = false;
        }
        // готовим результат
        EventGroupMonster {
            event_type: EventTypeMonster::AnswerLeavingGroup,
            id_sender: recipient,
            // меняем местами, т.к. теперь ВОЖДЬ шлет смску
            id_receiver: sender,
        }
    }

    // ВОЖДЬ шлет смс монстру, что он кикнут
    fn send_leave_event(&self,
                        temp_event: EventGroupMonster,
                        events_vec: &mut Vec<EventGroupMonster>,
    ) {
        // шлем члену сообщение что он выпилен из группы
        if temp_event.event_type != EventTypeMonster::None {
            events_vec.push(temp_event);
            println!("Подтверждаю выход монстра из группы");
        } else { println!("send_leave_event -> Что-то пошло не так, посылаем пустые события!"); };
    }


    // выход из группы монстра
    fn leaving_group(&self,
                     monster_attr: &mut MonsterAttributes,
    ) {
        // метим краба что он не в группе.
        monster_attr.in_group = false;
        monster_attr.id_lead = -1;
    }

    // ВОЖДЬ самовыпиливается из стаи
    fn leave_lead_in_group(&self,
                           monster_attr: &mut MonsterAttributes,
                           sender: i32,
    ) -> Vec<EventGroupMonster>
    {
        // выпиливаем отщепенца со списка группы
        monster_attr.group_members.retain(|&item| item != sender);
        // если список членов пуст, ВОЖДЬ уже не ВОЖДЬ =(
        if monster_attr.group_members.is_empty() {
            monster_attr.in_group = false;
            monster_attr.id_lead = -1;
            monster_attr.lead = false;
        }
        // готовим результат
        let mut vec_result: Vec<EventGroupMonster> = vec![];
        for group_member in &monster_attr.group_members {
            vec_result.push(EventGroupMonster {
                event_type: EventTypeMonster::AnswerLeavingGroup,
                id_sender: sender,
                // не меняем местами, особый случай
                id_receiver: *group_member,
            });
        };
        vec_result
    }

    // ВОЖДЬ шлет смс ВСЕМ, что он ушел
    fn send_leave_lead_event(&self,
                             temp_vec_event: Vec<EventGroupMonster>,
                             events_vec: &mut Vec<EventGroupMonster>,
    ) {
        // шлем члену сообщение что он выпилен из группы
        for temp_event in temp_vec_event {
            if temp_event.event_type != EventTypeMonster::None {
                events_vec.push(temp_event);
                println!("Подтверждаю выход монстра из группы");
            };
            println!("send_leave_lead_event -> Что-то пошло не так, пустые события!");
        }
    }
}