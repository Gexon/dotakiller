// будем юзать Behaviour Tree AI

use tinyecs::*;
use time::{PreciseTime, Duration};

use MONSTER_SPEED;

//use ::utility::map::Point;
use ::utility::enums::*;
use ::ground::components::Position;
use ::ground::components::ClassGround;
use ::ground::components::WindDirection;
use ::ground::components::WorldMap;
use ::ground::components::EventsTo;
use ::monster::components::*;
use ::monster::monster_graph::*;


/// Система восприятия
pub struct PerceptionSystem;
// тут типа чекает окружение, и помечает объекты что попадают в поле зения.
impl System for PerceptionSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround].optional(), aspect_all![MonsterClass].optional()]
    }

    // Вынимаем аспекты макросом, т.к. там безумие в коде.
    impl_process!( self, _entity, | _monster_class: MonsterClass | with (_ground, _monsters) => {
        // сканируем вокруг, может есть еда или вода или др. монстр или ОБОРИГЕН!

        // _monster_class - это: "let _monster_class = _entity.get_component::<MonsterClass>();
        let mut check_err: bool = false; // для предупреждения об одновременном сканировании.
        // проверяем таймеры, чтоб не спамить почем зря
        if _monster_class.perception_time.to(PreciseTime::now()) > Duration::seconds(MONSTER_SPEED) {

            {
                // палим растения, если голодны. NeedFood
                let behaviour_event = _entity.get_component::<BehaviourEvents>(); // события
                if behaviour_event.event.contains(&BehaviorEventEnum::NeedFood) {
                    check_err = true;
                    let mut monster_map = _entity.get_component::< MonsterMem > ();
                    let mut monster_state = _entity.get_component::< MonsterState > ();
                    let ground = &_ground[0];
                    let world_map = ground.get_component::< WorldMap > ();
                    // проверяем свободно ли место спавна.
                    let position = _entity.get_component::< Position > ();
                    //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
                    // очищаем цель
                    println!("PerceptionSystem - Ищем еды. Очищаем цель");
                    monster_map.action_target.target_type = TargetType::None;
                    monster_state.view_food = false;
                    'outer:
                    for x in - 2..3 {
                        // x от -2 до 0 и до 2, проверил на плейграунде.
                        for y in - 2..3 {
                                let pos_x: i32 = (position.x.trunc() as i32 + x) as i32;
                                let pos_y: i32 = (position.y.trunc() as i32 + y) as i32;
                                let scan_point: (i32, i32) = (pos_x, pos_y); // Casting
                                // Проверяем растет ли дерево по даденным координатам.
                                if !monster_state.view_food && world_map.flora.contains_key( & scan_point) {
                                    // добавляем растение в цель
                                    println!("PerceptionSystem - Ищем еды. Добавляем растение в цель");
                                    monster_map.action_target.target_type = TargetType::Flora;
                                    monster_map.action_target.position.x = pos_x as f32;
                                    monster_map.action_target.position.y = pos_y as f32;
                                    monster_state.view_food = true;
                                    break 'outer;
                                };
                        } // for
                    } // for
                } // BecomeHungry
            }


            {
                // Ищем ВОЖДЯ. NeedGroup
                let behaviour_event = _entity.get_component::<BehaviourEvents>(); // события
                if behaviour_event.event.contains(&BehaviorEventEnum::NeedGroup) && !check_err {
                    check_err = true;
                    //println!("Ищем ВОЖДЯ. NeedGroup");
                    let mut monster_mem = _entity.get_component::< MonsterMem > (); // это типа память, для хранения цели
                    let mut monster_state = _entity.get_component::< MonsterState > ();
                    let position = _entity.get_component::< Position > ();

                    //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
                    // очистка цели
                    monster_mem.action_target.target_type = TargetType::None;
                    monster_state.view_monster = false;
                    // перебираем всех монстров
                    for monster in _monsters {
                        if _entity.id == monster.id {continue};
                        let target_pos = monster.get_component::< Position > ();
                        let dist: i32 = ((position.x - target_pos.x).powf(2.0)+
                            (position.y - target_pos.y).powf(2.0)).sqrt() as i32;
                        if dist < 3 {
                            // добавляем монстра в цель
                            monster_mem.action_target.target_type = TargetType::Monster;
                            monster_mem.action_target.position.x = target_pos.x;
                            monster_mem.action_target.position.y = target_pos.y;
                            monster_state.view_monster = true;
                            break;
                        }
                    } // for monster
                } // Конец поиск ВОЖДЯ
                else if behaviour_event.event.contains(&BehaviorEventEnum::NeedGroup) && check_err {
                        println!("PerceptionSystem - Ошибка одновременного сканирования.");
                    };
            } // Конец  Ищем ВОЖДЯ. BecomeGroup


            if check_err {}; // заглушка чтоб не пиликало


            // фиксируем текущее время
            _monster_class.perception_time = PreciseTime::now();
        } // timer
    }); // макрос
} //


/// Генерация событий
// Создаем события, проверяем параметры.
pub struct EventSystem;

impl System for EventSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass, MonsterAttributes, BehaviourEvents)
    }

    fn process_one(&mut self, entity: &mut Entity) {
        let mut monster_class = entity.get_component::<MonsterClass>();
        if monster_class.event_time.to(PreciseTime::now()) > Duration::seconds(MONSTER_SPEED) {
            let mut behaviour_event = entity.get_component::<BehaviourEvents>(); // события
            let mut monster_state = entity.get_component::<MonsterState>();
            let monster_attr = entity.get_component::<MonsterAttributes>(); //
            let monster_map = entity.get_component::<MonsterMem>();
            let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки


            // реакция на обнаружение еды.
            if monster_state.view_food
                // Если в очереди нет такого события, то переключаем событие на Обнаружена еда.
                && !behaviour_event.event.contains(&BehaviorEventEnum::FoundFood) {
                // если в векторе behaviour_events.event, нет события FoundFood
                // то, добавляем его туда.
                behaviour_event.event.push(BehaviorEventEnum::FoundFood);
                //println!("Новое событие: монстр {} обнаружил еду!", monster_id.id);
                // убираем событие потери цели если видим еду.
                behaviour_event.event.retain(|x| x != &BehaviorEventEnum::TargetLost);
            } else if
                // если не видно цели, то:
                // проверяем наличие события обнаружения еды.
                behaviour_event.event.contains(&BehaviorEventEnum::FoundFood) {
                // если висит событие обнаружения еды, то убрать его из очереди событий
                behaviour_event.event.retain(|x| x != &BehaviorEventEnum::FoundFood);
                // todo лишняя проверка, убирать событие из очереди, в др месте.
            }

            // реакция на потерю цели.
            // если в цели ничего нет, то создать событие потери цели
            if monster_map.action_target.target_type == TargetType::None
                // если нет в событиях потери цели, то добавляем в очередь событий потерю цели
                && !behaviour_event.event.contains(&BehaviorEventEnum::TargetLost) {
                behaviour_event.event.push(BehaviorEventEnum::TargetLost);
            }

            // реакция на голод.
            if monster_attr.hungry < monster_attr.danger_hungry && !monster_state.low_food
                // наступает событие - ГОЛОД
                // danger_hungry = 960
                // 959+, 960
                && !behaviour_event.event.contains(&BehaviorEventEnum::BecomeHungry) {
                behaviour_event.event.push(BehaviorEventEnum::BecomeHungry);
                monster_state.low_food = true;
                println!("Новое событие: монстр {} голоден!", monster_id.id);
                // todo дублирующие друг друга переменные low_food и BecomeHungry
                // todo убрать с очереди EatFull
            }

            //реакция на усталость.
            if monster_attr.power < monster_attr.danger_power && !monster_state.low_power
                // наступает событие - УСТАЛ
                && !behaviour_event.event.contains(&BehaviorEventEnum::BecomeTired) {
                behaviour_event.event.push(BehaviorEventEnum::BecomeTired);
                monster_state.low_power = true;
                //println!("Новое событие: монстр {} устал!", monster_id.id);
                // todo дублирующие друг друга переменные low_power и BecomeTired
                // todo убрать из очереди PowerFull
            }

            // реакция на наелся
            if monster_state.low_food && (monster_attr.hungry > 990)
                // Монстр насытился
                && !behaviour_event.event.contains(&BehaviorEventEnum::EatFull) {
                behaviour_event.event.push(BehaviorEventEnum::EatFull);
                monster_state.low_food = false;
                //println!("Новое событие: монстр {} сыт!", monster_id.id);
                // todo заменить числовые значеия 990 на переменные подгружаемые с БД
                // todo вот тут нужно убирать BecomeHungry с очереди
            }

            // реакция на отдохнул
            if monster_state.low_power && (monster_attr.power > 990)
                && !behaviour_event.event.contains(&BehaviorEventEnum::PowerFull) {
                behaviour_event.event.push(BehaviorEventEnum::PowerFull);
                monster_state.low_power = false;
                //println!("Новое событие: монстр {} отдохнул!", monster_id.id);
                // todo тут убрать с очереди BecomeTired
            }

            // реакция на обнаружение другого монстра
            if monster_state.view_monster
                && !behaviour_event.event.contains(&BehaviorEventEnum::FoundMonster) {
                behaviour_event.event.push(BehaviorEventEnum::FoundMonster);
            }

            // фиксируем текущее время
            monster_class.event_time = PreciseTime::now();
        }
    }
}

/// Выбиральщик состояний дерева поведения
/// внутри функций не вынимать из сущности!!!!!!!!!!!!!!!!!!!! #сразукраш
// monster_state - внутри функций не вынимать из сущности!!!!!!!!!!!!!!!!!!!! #сразукраш
// используя код программатора SelectionTree, переключает состояния.
// используем выбиральщик, чтоб каждый узел выполнять за тик.
pub struct SelectorSystem;

impl System for SelectorSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass, SelectionTree)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround]]
    }

    fn process_d(&mut self, entity: &mut Entity, data: &mut DataList) {
        let mut monster_class = entity.get_component::<MonsterClass>();
        if monster_class.selector_time.to(PreciseTime::now()) > Duration::seconds(MONSTER_SPEED) {
            let mut selection_tree = entity.get_component::<SelectionTree>();
            // сумоним ветер
            let ground = data.unwrap_entity();
            let wind = ground.get_component::<WindDirection>();
            // дерево поведений собственной персоны
            let node: &mut NodeBehavior = &mut selection_tree.selector;
            // один тик,  обход дерева.
            exec_node(node, entity, &wind);
            //let status: Status = exec_node(node, entity, &wind);
            //println!("node.status {:?}, node.cursor {}", status, node.cursor);

            // фиксируем текущее время
            monster_class.selector_time = PreciseTime::now();
        }
    }
}

/// Bio Systems, будем умершвлять тут монстра.
// пересчет характеристик, значений жизнедеятельности монстра.
pub struct BioSystems;

impl System for BioSystems {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass, MonsterAttributes)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround]]
    }

    fn process_d(&mut self, entity: &mut Entity, data: &mut DataList) {
        let mut monster_class = entity.get_component::<MonsterClass>();
        if monster_class.bios_time.to(PreciseTime::now()) > Duration::seconds(2 * MONSTER_SPEED) {
            let ground = data.unwrap_entity();
            let mut monster_attr = entity.get_component::<MonsterAttributes>();
            let mut monster_map = entity.get_component::<MonsterMem>();
            let behaviour_state = entity.get_component::<BehaviourEvents>(); // состояние
            //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки

            // power
            if behaviour_state.action == BehaviorActions::Sleep {
                monster_attr.power += 5;
            } else {
                monster_attr.power -= monster_attr.speed;
            }

            // hungry
            if behaviour_state.action == BehaviorActions::Meal {
                // найти нужную пальму
                //let target = &mut (&mut monster_map.action_target);
                //let target = &monster_map.action_target;
                if monster_map.action_target.target_type == TargetType::Flora {
                    // запоминаем место хавки
                    monster_map.last_eating.x = monster_map.action_target.position.x;
                    monster_map.last_eating.y = monster_map.action_target.position.y;
                    //println!("запоминаем место хавки x{} y{}", monster_map.last_eating.x, monster_map.last_eating.y);
                    // очередь на уменьшение массы у пальмы
                    let mut events_to = ground.get_component::<EventsTo>();
                    events_to.events_eat_flora.push(
                        EventEatFlora {
                            value: 10,
                            x: monster_map.action_target.position.x,
                            y: monster_map.action_target.position.y,
                        });
                    //println!("event.push монстр {}", monster_id.id);
                    // наполняем монстру желудок
                    // todo переписат, необходимо подтверждение от пальмы/земли что его можно откусить
                    monster_attr.hungry += 10;
                }

                //println!("power {}, hungry {}, монстр {}", monster_attr.power, monster_attr.hungry, monster_id.id);
            } else if monster_attr.hungry > 0 {
                monster_attr.hungry -= 1;
            }


            // фиксируем текущее время
            monster_class.bios_time = PreciseTime::now();
        }
    }
}








































