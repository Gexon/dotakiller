// будем юзать Behaviour Tree AI

use tinyecs::*;
use time::{PreciseTime, Duration};

use MONSTER_SPEED;
use GROUND_SIZE;

use ::utility::map::Point;
use ::utility::enums::*;
use ::ground::components::Replication;
use ::ground::components::Position;
use ::ground::components::ClassGround;
use ::ground::components::WindDirection;
use ::ground::components::WorldMap;
use ::ground::components::EventsMonsterToFlora;
use ::monster::components::*;


/// Система восприятия
pub struct PerceptionSystem;
// тут типа чекает окружение, и помечает объекты что попадают в поле зения.
impl System for PerceptionSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass, MonsterState)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround]]
    }

    fn process_d(&mut self, entity: &mut Entity, data: &mut DataList) {
        // здесь тоже меняются события.

        // сканируем вокруг, может есть еда или вода или др. монстр или ОБОРИГЕН!
        // сканируем с интервалом равным перемещению.
        let mut monster_class = entity.get_component::<MonsterClass>();
        if monster_class.perception_time.to(PreciseTime::now()) > Duration::seconds(2 * MONSTER_SPEED) {
            let mut monster_map = entity.get_component::<MonsterMaps>();
            // сканируем окружность на предмет кактусов.
            //_MonsterMaps
            let mut monster_state = entity.get_component::<MonsterState>();
            let ground = data.unwrap_entity();
            let world_map = ground.get_component::<WorldMap>();
            // проверяем свободно ли место спавна.
            let position = entity.get_component::<Position>();
            //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
            'outer: for x in -2..3 {
                for y in -2..3 {
                    let pos_x: i32 = (position.x.trunc() + x as f32) as i32;
                    let pos_y: i32 = (position.y.trunc() + y as f32) as i32;
                    let scan_point: Point = Point(pos_x, pos_y); // Casting
                    // Проверяем растет ли дерево по даденным координатам.
                    if !monster_state.view_food && world_map.flora[scan_point] == 1 {
                        // добавляем растение в цель
                        monster_map.action_target.target_type = TargetType::Flora;
                        monster_map.action_target.position.x = pos_x as f32;
                        monster_map.action_target.position.y = pos_y as f32;
                        monster_state.view_food = true;
                        break 'outer;
                    } else {
                        // убираем растение из цели
                        monster_map.action_target.target_type = TargetType::None;
                        monster_state.view_food = false;
                    }
                }
            }


            // фиксируем текущее время
            monster_class.perception_time = PreciseTime::now();
        }
    }
}


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
            //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки

            // реакция на обнаружение еды.
            if monster_state.view_food
                // Если в очереди нет такого события, то переключаем событие на Обнаружена еда.
                && !behaviour_event.event.contains(&BehaviorEventEnum::FoundFood) {
                // если в векторе behaviour_events.event, нет события FoundFood
                // то, добавляем его туда.
                behaviour_event.event.push(BehaviorEventEnum::FoundFood);
                //println!("Новое событие: монстр {} обнаружил еду!", monster_id.id);
            } else if behaviour_event.event.contains(&BehaviorEventEnum::FoundFood) {
                // если еды нет в поле зрения и есть событие обнаружения еды,
                // то удалить событие из очереди.
                behaviour_event.event.retain(|x| x != &BehaviorEventEnum::FoundFood);
            }

            // реакция на голод.
            if monster_attr.hungry < monster_attr.danger_hungry && !monster_state.low_food
                // наступает событие - ГОЛОД
                // danger_hungry = 960
                // 959+, 960
                && !behaviour_event.event.contains(&BehaviorEventEnum::BecomeHungry) {
                behaviour_event.event.push(BehaviorEventEnum::BecomeHungry);
                monster_state.low_food = true;
                //println!("Новое событие: монстр {} голоден!", monster_id.id);
            }

            //реакция на усталость.
            if monster_attr.power < monster_attr.danger_power && !monster_state.low_power
                // наступает событие - УСТАЛ
                && !behaviour_event.event.contains(&BehaviorEventEnum::BecomeTired) {
                behaviour_event.event.push(BehaviorEventEnum::BecomeTired);
                monster_state.low_power = true;
                //println!("Новое событие: монстр {} устал!", monster_id.id);
            }


            // реакция на наелся
            if monster_state.low_food && (monster_attr.hungry > 990)
                // Монстр насытился
                && !behaviour_event.event.contains(&BehaviorEventEnum::EatFull) {
                behaviour_event.event.push(BehaviorEventEnum::EatFull);
                monster_state.low_food = false;
                //println!("Новое событие: монстр {} сыт!", monster_id.id);
            }


            // реакция на отдохнул
            if monster_state.low_power && (monster_attr.power > 990)
                && !behaviour_event.event.contains(&BehaviorEventEnum::PowerFull) {
                behaviour_event.event.push(BehaviorEventEnum::PowerFull);
                monster_state.low_power = false;
                //println!("Новое событие: монстр {} отдохнул!", monster_id.id);
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


/// Активатор. Приводит в действие.
// считывает текущее действие(экшон) и выполняет его.
pub struct _ActionSystem; // todo переименовать в ActionSystem

impl System for _ActionSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass, Position, MonsterState)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround]]
    }

    fn process_d(&mut self, _entity: &mut Entity, _data: &mut DataList) {
        // смотрит текущее действие и выполняет его.
        // тут заставляем монстра ходить, пить, искать.
        //let mut monster_state = entity.get_component::<MonsterState>();
        //if monster_state.behavior_time.to(PreciseTime::now()) > Duration::seconds(2 * MONSTER_SPEED) {
        //let mut selection_tree = entity.get_component::<SelectionTree>();
        //let behaviour_state = entity.get_component::<BehaviourState>(); // состояние
        //let mut monster_attr = entity.get_component::<MonsterAttributes>(); // атрибуты/характеристики
        //let mut position = entity.get_component::<Position>();
        //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
        // вынимаем узел из нашего графа поведения
        //let index: usize = selection_tree.cursor as usize;
        //let (_v_event, v_state, _v_status) = selection_tree.selector[index]; // (event, state, Status)
        //let mut r_status: Status = Status::Running;
        // сумоним ветер
        //let ground = data.unwrap_entity();
        //let wind = ground.get_component::<WindDirection>();
        //
        //let delta: f32 = monster_attr.speed as f32;


        //            match behaviour_state.state {
        //                BehaviorStateEnum::Sleep => {
        //                    println!("...zzz...монстр {}", monster_id.id);
        //                    if BehaviorStateEnum::from_index(v_state) == BehaviorStateEnum::Sleep {
        //                        r_status = Status::Success;
        //                    }
        //                }
        //
        //                BehaviorStateEnum::Walk => {
        //                    // тут заставляем монстра ходить туда-сюда, бесцельно, куда подует)
        //                    next_step(&mut position, delta, &wind);
        //                    entity.add_component(Replication); // произошли изменения монстра.
        //                    entity.refresh();
        //                    println!("монстр {} гуляет", monster_id.id);
        //                    println!("x:{}, y:{}, монстр {}", position.x, position.y, monster_id.id);
        //
        //                    if BehaviorStateEnum::from_index(v_state) == BehaviorStateEnum::Walk {
        //                        r_status = Status::Success;
        //                    }
        //                }
        //
        //                BehaviorStateEnum::FindFood => {
        //                    // поиск пищи. ходим по окружности
        //                    if monster_attr.speed == 1 { monster_attr.speed = 2 };
        //                    next_step_around(&mut position, monster_attr.speed as f32, &mut monster_state);
        //
        //                    entity.add_component(Replication); // произошли изменения монстра.
        //                    entity.refresh();
        //                    //println!("монстр {} ищет хавку", monster_id.id);
        //                    println!("x:{}, y:{}, монстр {}", position.x, position.y, monster_id.id);
        //                }
        //
        //                BehaviorStateEnum::Meal => {
        //                    monster_attr.hungry = 1000;
        //                    println!("монстр {} поел", monster_id.id);
        //                    if BehaviorStateEnum::from_index(v_state) == BehaviorStateEnum::Meal {
        //                        r_status = Status::Success;
        //                    }
        //                }
        //
        //                _ => {}
        //            }

        //let r_selector = (_v_event, v_state, r_status);
        //selection_tree.selector[index] = r_selector; // (event, state, Status)
        // фиксируем текущее время
        // monster_state.behavior_time = PreciseTime::now();
        //}
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
            let mut monster_map = entity.get_component::<MonsterMaps>();
            let behaviour_state = entity.get_component::<BehaviourEvents>(); // состояние
            let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки

            // power
            if behaviour_state.action == BehaviorActions::Sleep {
                monster_attr.power += 5;
            } else {
                monster_attr.power -= monster_attr.speed;
            }

            // hungry
            if behaviour_state.action == BehaviorActions::Meal {
                // найти нужную пальму
                let target = &mut (&mut monster_map.action_target);
                if target.target_type == TargetType::Flora {
                    // очередь на уменьшение массы у пальмы
                    let mut event_to_flora = ground.get_component::<EventsMonsterToFlora>();
                    event_to_flora.event.push(EventEatFlora {
                        value: 10,
                        x: target.position.x,
                        y: target.position.y,
                    });

                    // наполняем монстру желудок
                    monster_attr.hungry += 10;
                }

                println!("power {}, hungry {}, монстр {}", monster_attr.power, monster_attr.hungry, monster_id.id);
            } else if monster_attr.hungry > 0 {
                monster_attr.hungry -= 1;
            }


            // фиксируем текущее время
            monster_class.bios_time = PreciseTime::now();
        }
    }
}

// Поиск пути
/*
- для поиска пути, предложить проложить путь по шаблону(прямой путь, обход слева, обход справа), если шаблон не проходим, выполнить поиск пути.
Сначала нужно создать два списка: список узлов, которые еще не проверены (Unchecked), и список уже проверенных узлов (Checked). Каждый список включает узел расположения, предполагаемое расстояние до цели и ссылку на родительский объект (узел, который поместил данный узел в список). Изначально списки пусты.

Теперь добавим начальное расположение в список непроверенных, не указывая ничего в качестве родительского объекта. Затем вводим алгоритм.

    Выбираем в списке наиболее подходящий узел.
    Если этот узел является целью, то все готово.
    Если этот узел не является целью, добавляем его в список проверенных.
    Для каждого узла, соседнего с данным узлом.
        Если этот узел непроходим, игнорируем его.
        Если этот узел уже есть в любом из списков (проверенных или непроверенных), игнорируем его.
        В противном случае добавляем его в список непроверенных, указываем текущий узел в качестве родительского и рассчитываем длину пути до цели (достаточно просто вычислить расстояние).

Когда объект достигает поля цели, можно построить путь, отследив родительские узлы вплоть до узла, у которого нет родительского элемента (это начальный узел).

При загрузке большого количества запросов путей агент может либо подождать, либо, не дожидаясь выдачи путей, просто начать двигаться в нужном направлении (например, по алгоритму «Столкнуться и повернуть»).*/


/// Расчет следующего шага монстра.
// Вывезли наружу расчет следующего шага монстра.
pub fn next_step(position: &mut Position, delta: f32, wind: &WindDirection) {
    match wind.direction {
        Direction::North => {
            if position.x < (GROUND_SIZE as f32 - delta) {
                position.x += delta;
            }
        }
        Direction::NorthWest => {
            if (position.x > position.y * 2f32) & &(position.x < (GROUND_SIZE as f32 - delta)) {
                position.x += delta;
            }
        }
        Direction::West => {
            if position.y > 0f32 + delta {
                position.y -= delta;
            }
        }
        Direction::WestSouth => {
            if (position.y < position.x * 2f32) & &(position.y > 0f32 + delta) {
                position.y -= delta;
            }
        }
        Direction::South => {
            if position.x > 0f32 + delta {
                position.x -= delta;
            }
        }
        Direction::SouthEast => {
            if (position.x < position.y * 2f32) & &(position.x > 0f32 + delta) {
                position.x -= delta;
            }
        }
        Direction::East => {
            if position.y < (GROUND_SIZE as f32 - delta) {
                position.y += delta;
            }
        }
        Direction::EastNorth => {
            if (position.y > position.x * 2f32) & &(position.y < (GROUND_SIZE as f32 - delta)) {
                position.y += delta;
            }
        }
    }
}


/// Расчет следующего шага по кругу монстра.
// По кругу должен ходить.
pub fn next_step_around(position: &mut Position, delta: f32, monster_state: &mut MonsterState) {
    // проверяем достижение цели
    if (position.x as u32 == monster_state.target_point.x as u32)
        & &(position.y as u32 == monster_state.target_point.y as u32) {
        // цель достигнута, ставим новую цель:
        // выбираем новую цель по направлению монстра.
        // две клетки вперед и одну вправо. меняем направление монстра.
        match monster_state.move_target.direct {
            Direction::North | Direction::NorthWest => {
                if (position.x as u32) < GROUND_SIZE - 2 { monster_state.target_point.x += 2f32; };
                if (position.y as u32) > 1 { monster_state.target_point.y -= 1f32; };
                monster_state.move_target.direct = Direction::West;
            }

            Direction::West | Direction::WestSouth => {
                if (position.y as u32) > 2 { monster_state.target_point.y -= 2f32; };
                if (position.x as u32) > 1 { monster_state.target_point.x -= 1f32; };
                monster_state.move_target.direct = Direction::South;
            }

            Direction::South | Direction::SouthEast => {
                if (position.x as u32) > 2 { monster_state.target_point.x -= 2f32; };
                if (position.y as u32) < GROUND_SIZE - 1 { monster_state.target_point.y += 1f32; };
                monster_state.move_target.direct = Direction::East;
            }

            Direction::East | Direction::EastNorth => {
                if (position.y as u32) < GROUND_SIZE - 2 { monster_state.target_point.y += 2f32; };
                if (position.x as u32) < GROUND_SIZE - 1 { monster_state.target_point.x += 1f32; };
                monster_state.move_target.direct = Direction::North;
            }
        }
        println!("NEW target_x {}, target_y {}", monster_state.target_point.x, monster_state.target_point.y);
    } else {
        // цель не достигнута, делаем шаг в сторону цели.
        // расчеты по Х
        if position.x < monster_state.target_point.x {
            monster_state.old_position.x = position.x;
            if position.x + delta > monster_state.target_point.x {
                position.x = monster_state.target_point.x
            } else { position.x += delta };
        } else if position.x > monster_state.target_point.x {
            monster_state.old_position.x = position.x;
            if position.x - delta < monster_state.target_point.x {
                position.x = monster_state.target_point.x
            } else { position.x -= delta };
        }

        // расчеты по Y
        if position.y < monster_state.target_point.y {
            monster_state.old_position.y = position.y;
            if position.y + delta > monster_state.target_point.y {
                position.y = monster_state.target_point.y
            } else { position.y += delta };
        } else if position.y > monster_state.target_point.y {
            monster_state.old_position.y = position.y;
            if position.y - delta < monster_state.target_point.y {
                position.y = monster_state.target_point.y
            } else { position.y -= delta };
        }
    }
}


/// Обход вектора
pub fn exec_node(branch: &mut NodeBehavior, entity: &Entity, wind: &WindDirection) -> Status {
    //println!(">>> exec_node");
    // при первом проходе, будет 1 узел NodeBehavior
    // внутри него behavior = Sequencer
    // Sequencer содержит в себе вектор из NodeBehavior.
    let cursor: &mut usize = &mut branch.cursor;
    let behavior: &mut BehaviorEnum = &mut branch.behavior;
    match *behavior {
        // последовательность, до первого узла Fail, либо выполняет все и возвращает Success
        BehaviorEnum::Sequencer(ref mut vec_sequence) => {
            //println!("Sequencer cursor:{}", *cursor);
            for (index, node_beh) in vec_sequence.iter_mut().enumerate().skip(*cursor) {
                //if index >= cursor { //.skip(cursor) пропускает все до cursor
                *cursor = index; // пометить узел на котором мы торчим
                let node_branch: &mut NodeBehavior = &mut (*node_beh);
                //println!("index {}", index);
                let status = exec_node(node_branch, entity, wind);
                // что делать если нам вернули что процесс еще выполняется?
                // прерываем и выходим, запоминая курсор.
                if status == Status::Running {
                    //println!("Running Sequencer cursor {}", *cursor);
                    return status
                }
                //}
            }
            *cursor = 0; // успех или провал, обнуляем курсор, т.к. начинать полюбасу поновой)))
            //println!("return2 Sequencer cursor {}", cursor);
            Status::Success
        }

        //
        BehaviorEnum::If(ref mut beh_enum1, ref mut beh_enum2, ref mut beh_enum3) => {
            //println!("If cursor:{}", *cursor);
            match *cursor {
                1 => {
                    let status = exec_node(beh_enum2, entity, wind);
                    if status != Status::Running { *cursor = 0 }
                    status
                }
                2 => {
                    let status = exec_node(beh_enum3, entity, wind);
                    if status != Status::Running { *cursor = 0 }
                    status
                }
                _ => {
                    // Курсор хранит значение последнего выбранного варианта.
                    let status = exec_node(beh_enum1, entity, wind);
                    match status {
                        Status::Success => {
                            let status_if = exec_node(beh_enum2, entity, wind);
                            if status_if == Status::Running { *cursor = 1 } else { *cursor = 0 }
                            status_if
                        }

                        Status::Failure => {
                            let status_if = exec_node(beh_enum3, entity, wind);
                            if status_if == Status::Running { *cursor = 2 } else { *cursor = 0 }
                            status_if
                        }

                        Status::Running => {
                            Status::Running
                        }
                    }
                }
            }
        }

        //
        BehaviorEnum::Action(beh_act) => {
            //
            exec_action(beh_act, entity, wind)
        }

        _ => { Status::Success }
    }
}


/// Активация действий монстра
pub fn exec_action(action: BehaviorActions, entity: &Entity, wind: &WindDirection) -> Status {
    //println!("exec_action {:?}", action);
    match action {
        // 0. ХЗ, резервёд Ё
        BehaviorActions::Null => run_null(entity),
        // Проверить усталось.
        BehaviorActions::CheckTired => run_check_tired(entity),
        // Сон. Монстр ждет, в этот момент с ним ничего не происходит.
        BehaviorActions::Sleep => run_sleep(entity),
        // Бодрствование. Случайное перемещение по полигону.
        BehaviorActions::Walk => run_walk(entity, wind),
        // Проверить голоден ли.
        BehaviorActions::CheckHungry => run_check_hungry(entity),
        // Поиск пищи.(ХЗ, сложная хрень и долгая)
        BehaviorActions::FindFood => run_find_food(entity),
        // Поиск воды.
        BehaviorActions::FindWater => run_find_water(entity),
        // Прием пищи.
        BehaviorActions::Meal => run_meal(entity),
        // Прием воды.
        BehaviorActions::WaterIntake => run_water_intake(entity),
        // Перемещение к цели.
        BehaviorActions::MoveToTarget => run_move_to_target(entity),
    }
}


/// Действие монстра "инициализация"
pub fn run_null(_entity: &Entity) -> Status {
    //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    //println!("Init монстр {}", monster_id.id);
    Status::Success
}


/// Действие монстра "спать"
pub fn run_sleep(entity: &Entity) -> Status {
    let mut behaviour_state = entity.get_component::<BehaviourEvents>(); // состояние
    //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки

    // отдохнул
    if behaviour_state.event.contains(&BehaviorEventEnum::PowerFull) {
        behaviour_state.event.retain(|x| x != &BehaviorEventEnum::PowerFull);
        behaviour_state.event.retain(|x| x != &BehaviorEventEnum::BecomeTired);
        behaviour_state.action = BehaviorActions::Null;
        //println!("монстр {} отдохнул", monster_id.id);
        return Status::Success
    }

    behaviour_state.action = BehaviorActions::Sleep;
    //println!("...zzz...монстр {}", monster_id.id);
    Status::Running
}


/// Действие монстра "проверить усталость"
pub fn run_check_tired(entity: &Entity) -> Status {
    let behaviour_event = entity.get_component::<BehaviourEvents>(); // события
    //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    //println!("монстр {} проверяет усталость", monster_id.id);
    if behaviour_event.event.contains(&BehaviorEventEnum::BecomeTired) {
        //println!("монстр устал");
        return Status::Success
    }
    //println!("монстр бодр");
    Status::Failure
}


/// Действие монстра "проверить голод"
pub fn run_check_hungry(entity: &Entity) -> Status {
    let behaviour_event = entity.get_component::<BehaviourEvents>(); // события
    //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    //println!("монстр {} проверяет голод", monster_id.id);
    if behaviour_event.event.contains(&BehaviorEventEnum::BecomeHungry) {
        //println!("монстр голоден");
        return Status::Success
    }
    //println!("монстр сыт");
    Status::Failure
}


/// Действие монстра "искать воду"
pub fn run_find_water(entity: &Entity) -> Status {
    let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    println!("...zzz...монстр {}", monster_id.id);
    Status::Success
}


/// Действие монстра "прием воды"
pub fn run_water_intake(entity: &Entity) -> Status {
    let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    println!("...zzz...монстр {}", monster_id.id);
    Status::Success
}


/// Действие монстра "движение к цели"
pub fn run_move_to_target(entity: &Entity) -> Status {
    let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    println!("...zzz...монстр {}", monster_id.id);
    Status::Success
}


/// Действие монстра "гулять"
pub fn run_walk(entity: &Entity, wind: &WindDirection) -> Status {
    let monster_attr = entity.get_component::<MonsterAttributes>(); // атрибуты/характеристики
    let mut behaviour_state = entity.get_component::<BehaviourEvents>(); // состояние
    let mut position = entity.get_component::<Position>();
    //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    let delta: f32 = monster_attr.speed as f32;
    // тут заставляем монстра ходить туда-сюда, бесцельно, куда подует)
    next_step(&mut position, delta, wind);
    behaviour_state.action = BehaviorActions::Walk;
    entity.add_component(Replication); // произошли изменения монстра.
    entity.refresh();
    //println!("Монстр {} гуляет x:{}, y:{},", monster_id.id, position.x, position.y);
    Status::Success
}


/// Действие монстра "искать еду"
pub fn run_find_food(entity: &Entity) -> Status {
    let mut monster_attr = entity.get_component::<MonsterAttributes>(); // атрибуты/характеристики
    let mut monster_state = entity.get_component::<MonsterState>();
    let mut behaviour_event = entity.get_component::<BehaviourEvents>(); // события
    let mut position = entity.get_component::<Position>();
    //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    // поиск пищи. ходим по окружности
    if monster_attr.speed == 1 { monster_attr.speed = 2 };
    next_step_around(&mut position, monster_attr.speed as f32, &mut monster_state);
    //
    entity.add_component(Replication); // произошли изменения монстра.
    entity.refresh();

    if behaviour_event.event.contains(&BehaviorEventEnum::FoundFood) {
        // Те элементы, которые не удовлетворяют предикату удаляются!
        /*let mut vec = vec![1, 2, 3, 4];
        vec.retain(|&x| x%2 == 0);
        assert_eq!(vec, [2, 4]);*/
        behaviour_event.event.retain(|x| x != &BehaviorEventEnum::FoundFood);
        //println!("Монстр {} вижу пальму", monster_id.id);
        return Status::Success
    }

    //println!("Монстр {} ищет че бы поесть", monster_id.id);
    Status::Running
}


/// Действие монстра "трапезничать"
pub fn run_meal(entity: &Entity) -> Status {
    let mut behaviour_event = entity.get_component::<BehaviourEvents>(); // события
    //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки


    // наелся
    if behaviour_event.event.contains(&BehaviorEventEnum::EatFull) {
        behaviour_event.event.retain(|x| x != &BehaviorEventEnum::EatFull);
        behaviour_event.event.retain(|x| x != &BehaviorEventEnum::BecomeHungry);
        behaviour_event.action = BehaviorActions::Null;
        //println!("монстр {} наелся", monster_id.id);
        return Status::Success
    }

    // пальму съесть
    behaviour_event.action = BehaviorActions::Meal;
    //println!("монстр {} ест", monster_id.id);
    Status::Running
}