// модуль обхода графа
use tinyecs::*;

use GROUND_SIZE;

use ::ground::components::Replication;
use ::ground::components::WindDirection;
use ::ground::components::Position;
use ::ground::components::EventsTo;
use ::monster::components::*;
use ::utility::enums::*;


/// Обход графа
// Нельзя допускать одновременое исполнение всех веток !
pub fn exec_node(branch: &mut NodeBehavior, entity: &Entity, ground: &Entity) -> Status {
    //println!(">>> exec_node");
    // при первом проходе, будет 1 узел NodeBehavior
    // внутри него behavior = Sequencer
    // Sequencer содержит в себе вектор из NodeBehavior.
    let cursor: &mut usize = &mut branch.cursor;
    let behavior: &mut NodeType = &mut branch.behavior;
    match *behavior {
        // последовательность, до первого узла Fail, либо выполняет все и возвращает Success
        NodeType::Sequencer(ref mut vec_sequence) => {
            //println!("ROOT cursor:{}", *cursor);
            for (index, node_beh) in vec_sequence.iter_mut().enumerate().skip(*cursor) {
                //.skip(cursor) пропускает все до cursor
                //println!("Sequencer cursor:{}", index);
                *cursor = index; // пометить узел на котором мы торчим
                let node_branch: &mut NodeBehavior = &mut (*node_beh);
                let status = exec_node(node_branch, entity, ground);
                // что делать если нам вернули что процесс еще выполняется?
                // прерываем и выходим, запоминая курсор.
                if status == Status::Running {
                    //println!("RUNNING Sequencer cursor {}", *cursor);
                    return status;
                }
            }
            *cursor = 0; // успех или провал, обнуляем курсор, т.к. начинать полюбасу поновой)))
            //println!("RETURN Sequencer cursor {}", cursor);
            Status::Success
        }

        //
        NodeType::If(ref mut beh_enum1, ref mut beh_enum2, ref mut beh_enum3) => {
            //println!("If cursor:{}", *cursor);
            match *cursor {
                1 => {
                    let status = exec_node(beh_enum2, entity, ground);
                    if status != Status::Running { *cursor = 0 }
                    status
                }
                2 => {
                    let status = exec_node(beh_enum3, entity, ground);
                    if status != Status::Running { *cursor = 0 }
                    status
                }
                _ => {
                    // Курсор хранит значение последнего выбранного варианта.
                    let status = exec_node(beh_enum1, entity, ground);
                    match status {
                        Status::Success => {
                            let status_if = exec_node(beh_enum2, entity, ground);
                            if status_if == Status::Running { *cursor = 1 } else { *cursor = 0 }
                            status_if
                        }

                        Status::Failure => {
                            let status_if = exec_node(beh_enum3, entity, ground);
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
        NodeType::Action(beh_act) => {
            //
            exec_action(beh_act, entity, ground)
        }

        _ => { Status::Success }
    }
}

/// Активация действий монстра
fn exec_action(action: BehaviorActions, entity: &Entity, ground: &Entity) -> Status {
    //println!("exec_action {:?}", action);
    match action {
        // 0. ХЗ, резервёд Ё
        BehaviorActions::Null => run_null(entity),
        // Проверить усталось.
        BehaviorActions::CheckTired => run_check_tired(entity),
        // Сон. Монстр ждет, в этот момент с ним ничего не происходит.
        BehaviorActions::Sleep => run_sleep(entity),
        // Бодрствование. Случайное перемещение по полигону.
        BehaviorActions::Walk => run_walk(entity, ground),
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
        // Проверка в памяти инф о еде
        BehaviorActions::CheckMemMeal => run_check_memory_meal(entity),
        // поиск монстра
        BehaviorActions::FindMonsters => run_find_monsters(entity),
        // Проверяем в группе ли монстр.
        BehaviorActions::CheckInGroup => run_check_in_group(entity, ground),
        // Вступаем в группу.
        BehaviorActions::JoinGroup => run_join_to_group(entity, ground),
    }
}

/// Действие монстра "инициализация"
fn run_null(_entity: &Entity) -> Status {
    //let monster_id = _entity.get_component::<MonsterId>(); // удалить. для отладки
    //println!("run_null монстр {}", monster_id.id);
    Status::Success
}

/// Действие монстра "проверить усталость"
fn run_check_tired(entity: &Entity) -> Status {
    let behaviour_event = entity.get_component::<BehaviourEvents>(); // события
    let mut state = entity.get_component::<MonsterState>(); // состояние монстра
    //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    //println!("монстр {} проверяет усталость", monster_id.id);
    if behaviour_event.event.contains(&BehaviorEventEnum::BecomeTired) {
        //println!("монстр устал");
        state.emo_state = 2;
        entity.add_component(Replication); // произошли изменения монстра.
        entity.refresh();
        return Status::Success;
    }
    //println!("монстр бодр");
    state.emo_state = 0;
    Status::Failure
}

/// Действие монстра "спать"
fn run_sleep(entity: &Entity) -> Status {
    let mut behaviour_state = entity.get_component::<BehaviourEvents>(); // состояние
    let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки

    // отдохнул
    if behaviour_state.event.contains(&BehaviorEventEnum::PowerFull) {
        behaviour_state.event.retain(|x| x != &BehaviorEventEnum::PowerFull);
        behaviour_state.event.retain(|x| x != &BehaviorEventEnum::BecomeTired);
        behaviour_state.action = BehaviorActions::Null;
        //println!("монстр {} отдохнул", monster_id.id);
        return Status::Success;
    }

    behaviour_state.action = BehaviorActions::Sleep;
    println!("...zzz...монстр {}", monster_id.id);
    Status::Running
}

/// Действие монстра "гулять"
fn run_walk(entity: &Entity, ground: &Entity) -> Status {
    let wind = ground.get_component::<WindDirection>();
    let monster_attr = entity.get_component::<MonsterAttributes>(); // атрибуты/характеристики
    let mut behaviour_state = entity.get_component::<BehaviourEvents>(); // состояние
    let mut position = entity.get_component::<Position>();
    //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    let delta: f32 = monster_attr.speed as f32;
    // тут заставляем монстра ходить туда-сюда, бесцельно, куда подует)
    next_step_calculate(&mut position, delta, &wind);
    behaviour_state.action = BehaviorActions::Walk;
    entity.add_component(Replication); // произошли изменения монстра.
    entity.refresh();
    //println!("Монстр гуляет", );
    Status::Success
}

/// Действие монстра "проверить голод"
fn run_check_hungry(entity: &Entity) -> Status {
    let mut behaviour_event = entity.get_component::<BehaviourEvents>(); // события
    let mut state = entity.get_component::<MonsterState>(); // состояние монстра
    //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    //println!("монстр {} проверяет голод", monster_id.id);

    //

    if behaviour_event.event.contains(&BehaviorEventEnum::BecomeHungry) {
        // NeedFood - для активизаци сканирования вокруг чебя.
        if !behaviour_event.event.contains(&BehaviorEventEnum::NeedFood) {
            //println!("Создаем монстру событие NeedFood");
            behaviour_event.event.push(BehaviorEventEnum::NeedFood);
        }
        println!("монстр голоден");
        state.emo_state = 1;
        entity.add_component(Replication); // произошли изменения монстра.
        entity.refresh();
        return Status::Success;
    }
    //println!("монстр сыт");
    Status::Failure
}

/// Действие монстра "искать еду"
fn run_find_food(entity: &Entity) -> Status {
    let mut monster_attr = entity.get_component::<MonsterAttributes>(); // атрибуты/характеристики
    let mut monster_state = entity.get_component::<MonsterState>();
    let mut behaviour_event = entity.get_component::<BehaviourEvents>(); // события
    let mut position = entity.get_component::<Position>();
    let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки

    // если мы долго кружим
    if monster_state.find_around_count > 8 {
        monster_state.find_around_count = 0;
        println!("Долго ищем еду, прекратить поиски");
        if behaviour_event.event.contains(&BehaviorEventEnum::NeedFood) {
            //println!("Выпиливаем монстру событие NeedFood");
            behaviour_event.event.retain(|x| x != &BehaviorEventEnum::NeedFood);
        }
        return Status::Failure;
    } else {
        monster_state.find_around_count += 1;
    }

    if behaviour_event.event.contains(&BehaviorEventEnum::FoundFood) {
        // есть событие обнаружена еда, убираем событие из списка событий.
        behaviour_event.event.retain(|x| x != &BehaviorEventEnum::FoundFood);
        println!("Монстр {} вижу пальму", monster_id.id);
        monster_attr.speed = 1;
        return Status::Success;
    }

    println!("Монстр {} ищет че бы поесть", monster_id.id);
    // поиск пищи. ходим по окружности
    if monster_attr.speed == 1 { monster_attr.speed = 2 };
    next_step_around(&mut position, monster_attr.speed as f32, &mut monster_state);
    entity.add_component(Replication); // произошли изменения монстра.
    entity.refresh();
    Status::Running
}

/// Действие монстра "искать воду"
fn run_find_water(_entity: &Entity) -> Status {
    //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    //println!("...zzz...монстр {}", monster_id.id);
    Status::Success
}

/// Действие монстра "трапезничать"
fn run_meal(entity: &Entity) -> Status {
    let mut behaviour_event = entity.get_component::<BehaviourEvents>(); // события
    let mut state = entity.get_component::<MonsterState>(); // состояние монстра
    let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки

    // наелся
    if behaviour_event.event.contains(&BehaviorEventEnum::EatFull) {
        behaviour_event.event.retain(|x| x != &BehaviorEventEnum::EatFull);
        behaviour_event.event.retain(|x| x != &BehaviorEventEnum::BecomeHungry);
        //println!("Выпиливаем монстру событие NeedFood");
        behaviour_event.event.retain(|x| x != &BehaviorEventEnum::NeedFood);
        behaviour_event.action = BehaviorActions::Null;
        println!("монстр {} наелся", monster_id.id);
        state.emo_state = 0;
        entity.add_component(Replication); // произошли изменения монстра.
        entity.refresh();
        return Status::Success;
    } else if behaviour_event.event.contains(&BehaviorEventEnum::TargetLost) {
        println!("монстр {} потерял цель", monster_id.id);
        behaviour_event.event.retain(|x| x != &BehaviorEventEnum::TargetLost);
        //println!("Выпиливаем монстру событие NeedFood");
        behaviour_event.event.retain(|x| x != &BehaviorEventEnum::NeedFood); // убираем чтоб не возникло ошибки сканирования.
        return Status::Failure;
    }

    // пальму съесть
    behaviour_event.action = BehaviorActions::Meal;
    println!("монстр {} ест", monster_id.id);
    Status::Running
}

/// Действие монстра "прием воды"
fn run_water_intake(_entity: &Entity) -> Status {
    //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    //println!("...zzz...монстр {}", monster_id.id);
    Status::Success
}


/// Действие монстра "движение к цели"
fn run_move_to_target(entity: &Entity) -> Status {
    let monster_attr = entity.get_component::<MonsterAttributes>(); // атрибуты/характеристики
    let mut monster_state = entity.get_component::<MonsterState>(); // атрибуты/характеристики
    let mut position = entity.get_component::<Position>();
    //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    let delta = monster_attr.speed as f32;
    //let x_tpoint = monster_state.target_point.x;
    //let y_tpoint = monster_state.target_point.y;
    //println!("монстр {} идет к цели: x{} y{}", monster_id.id, x_tpoint, y_tpoint, );
    // проверяем достижение цели
    if (position.x as u32 == monster_state.target_point.x as u32)
        && (position.y as u32 == monster_state.target_point.y as u32) {
        // цель достигнута,
        //println!("монстр {} добрался до цели", monster_id.id);
        monster_state.find_around_count = 0;
        return Status::Success;
    } else {
        // цель не достигнута, делаем шаг в сторону цели.
        next_step_make(&mut position, monster_state.target_point, &mut monster_state, delta);
    }

    entity.add_component(Replication); // произошли изменения монстра.
    entity.refresh();

    Status::Running
}

/// Действие монстра "пошарить в памяти в поисках последнего места еды"
fn run_check_memory_meal(entity: &Entity) -> Status {
    let mut monster_state = entity.get_component::<MonsterState>();
    let monster_maps = entity.get_component::<MonsterMem>();
    //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    if monster_maps.last_eating.x < 0f32 {
        //println!("монстр {} не помнит где раньше питался", monster_id.id);
        return Status::Failure;
    }

    monster_state.target_point.x = monster_maps.last_eating.x;
    monster_state.target_point.y = monster_maps.last_eating.y;
    //println!("монстр {} вспомнил где ел, x{} y{}", monster_id.id, monster_maps.last_eating.x, monster_maps.last_eating.y);
    Status::Success
}


/// проверяет в группе ли монстр
fn run_check_in_group(entity: &Entity, ground: &Entity) -> Status {
    // MonsterAttributes
    let monster_attr = entity.get_component::<MonsterAttributes>(); // атрибуты/характеристики
    let mut behaviour_event = entity.get_component::<BehaviourEvents>(); // события
    let mut monster_state = entity.get_component::<MonsterState>(); // атрибуты/характеристики
    let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    if monster_attr.in_group {
        if !monster_attr.lead {
            //println!("Монстр {} в группе, ВОЖДЬ {}", monster_id.id, monster_attr.id_lead);
            // добавляем в очередь событий запрос на обновление координат ВОЖДЯ
            //println!("монстр {} обновляет координаты ВОЖДЯ", monster_id.id, );
            let mut events_to = ground.get_component::<EventsTo>();
            events_to.events_group_monster.push(
                EventGroupMonster {
                    event_type: EventTypeMonster::UpdateLeadPoint,
                    id_sender: monster_id.id,
                    id_receiver: monster_attr.id_lead,
                });
            // идем за вождем
            if monster_state.lead_point.is_some() {
                let mut position = entity.get_component::<Position>();
                move_to_lead(entity, &mut position, &mut monster_state, &monster_id, monster_attr.speed as f32);
            } else {println!("monster_state.lead_point - Пусто")}
            /*
            if let Some(ref _x) = monster_state.lead_point {
                // use _x
            }
            */
        } else {
            //println!("Монстр {} ВОЖДЬ", monster_id.id);
        }

        return Status::Success;
    }
    // NeedGroup
    if !behaviour_event.event.contains(&BehaviorEventEnum::NeedGroup) {
        println!("Создаем монстру событие NeedGroup");
        behaviour_event.event.push(BehaviorEventEnum::NeedGroup);
    }
    println!("Монстр не в группе");
    Status::Failure
}


/// ищем других монстров вокруг себя
fn run_find_monsters(entity: &Entity) -> Status {
    //println!("run_find_monsters - start");
    let mut monster_attr = entity.get_component::<MonsterAttributes>(); // атрибуты/характеристики
    let mut monster_state = entity.get_component::<MonsterState>();
    let mut behaviour_event = entity.get_component::<BehaviourEvents>(); // события
    let mut position = entity.get_component::<Position>();
    let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки

    // если мы долго кружим
    if monster_state.find_around_count > 4 {
        monster_state.find_around_count = 0;
        println!("Монстр {} долго кружит в поисках друзей", monster_id.id);
        // Выпиливаем NeedGroup, чтоб не сканировало
        if behaviour_event.event.contains(&BehaviorEventEnum::NeedGroup) {
            println!("Выпиливаем монстру событие NeedGroup");
            behaviour_event.event.retain(|x| x != &BehaviorEventEnum::NeedGroup);
            behaviour_event.event.retain(|x| x != &BehaviorEventEnum::TargetLost);
        }

        return Status::Failure;
    } else {
        monster_state.find_around_count += 1;
    }

    if behaviour_event.event.contains(&BehaviorEventEnum::FoundMonster) {
        // есть событие обнаружен монстр, убираем событие из списка событий.
        behaviour_event.event.retain(|x| x != &BehaviorEventEnum::FoundMonster);
        println!("Монстр {} видит коллегу", monster_id.id);
        println!("Выпиливаем монстру событие NeedGroup");
        behaviour_event.event.retain(|x| x != &BehaviorEventEnum::NeedGroup);
        monster_attr.speed = 1;
        return Status::Success;
    }

    println!("Монстр {} ищет соратников", monster_id.id);
    // поиск другого монстра. ходим по окружности
    if monster_attr.speed == 1 { monster_attr.speed = 2 };
    next_step_around(&mut position, monster_attr.speed as f32, &mut monster_state);
    entity.add_component(Replication); // произошли изменения монстра.
    entity.refresh();
    Status::Running
}


/// вступаем в группу
fn run_join_to_group(entity: &Entity, ground: &Entity) -> Status {
    let monster_id = entity.get_component::<MonsterId>();
    let monster_mem = entity.get_component::<MonsterMem>(); // это типа память, для хранения цели
    // добавляем в очередь событий просьбу вступления в группу.
    println!("монстр {} шлет просьбу принять в группу монстру {}", monster_id.id, monster_mem.action_target.target_id);
    let mut events_to = ground.get_component::<EventsTo>();
    events_to.events_group_monster.push(
        EventGroupMonster {
            event_type: EventTypeMonster::RequestJoinGroup,
            id_sender: monster_id.id,
            id_receiver: monster_mem.action_target.target_id,
        });

    Status::Failure
}


// -------------------------------------------------------------------------------------------------
// КОНЕЦ основных функций графа мсье
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------------------

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
fn next_step_calculate(position: &mut Position, delta: f32, wind: &WindDirection) {
    match wind.direction {
        Direction::North => {
            if position.x < (GROUND_SIZE as f32 - delta) {
                position.x += delta;
            }
        }
        Direction::NorthWest => {
            if (position.x > position.y * 2f32) && (position.x < (GROUND_SIZE as f32 - delta)) {
                position.x += delta;
            }
        }
        Direction::West => {
            if position.y > 0f32 + delta {
                position.y -= delta;
            }
        }
        Direction::WestSouth => {
            if (position.y < position.x * 2f32) && (position.y > 0f32 + delta) {
                position.y -= delta;
            }
        }
        Direction::South => {
            if position.x > 0f32 + delta {
                position.x -= delta;
            }
        }
        Direction::SouthEast => {
            if (position.x < (position.y * 2f32)) && (position.x > 0f32 + delta) {
                position.x -= delta;
            }
        }
        Direction::East => {
            if position.y < (GROUND_SIZE as f32 - delta) {
                position.y += delta;
            }
        }
        Direction::EastNorth => {
            if (position.y > position.x * 2f32) && (position.y < (GROUND_SIZE as f32 - delta)) {
                position.y += delta;
            }
        }
    }
}


/// Двигаем монстра на один шаг дальше
fn next_step_make(
    position: &mut ::ground::components::Position,
    target: ::monster::components::PositionM,
    monster_state: &mut ::monster::components::MonsterState,
    delta: f32,
) {
    // расчеты по Х
    if position.x < target.x {
        monster_state.old_position.x = position.x;

        position.x = f32::min(position.x + delta, target.x);
    } else if position.x > target.x {
        monster_state.old_position.x = position.x;

        position.x = f32::max(position.x - delta, target.x);
    }

    // расчеты по Y
    if position.y < target.y {
        monster_state.old_position.y = position.y;

        position.y = f32::min(position.y + delta, target.y);
    } else if position.y > target.y {
        monster_state.old_position.y = position.y;

        position.y = f32::max(position.y - delta, target.y);
    }
}


/// Расчет следующего шага по кругу монстра.
// По кругу должен ходить.
fn _next_step_around2(position: &mut Position, monster_state: &mut MonsterState) {
    // алгоритм движения по кругу для клетчатого поля
    // https://stackoverflow.com/questions/398299/looping-in-a-spiral
    let delta_x = monster_state.delta_x;
    let delta_y = monster_state.delta_y;
    let mut x: i32 = position.x as i32;
    let mut y: i32 = position.y as i32;
    let mut dx = delta_x;
    let mut dy = delta_y;

    // середа спирали в начале координат (0.0). Вроде =)
    if (x == y) || ((x < 0) && (x == -y)) || ((x > 0) && (x == 1 - y)) {
        let t = dx;
        dx = -dy;
        dy = t;
    };

    x += dx;
    y += dy;
    monster_state.delta_x = dx;
    monster_state.delta_y = dy;

    if ((position.x as u32) < GROUND_SIZE)
        && ((position.x as u32) > 0)
        && ((position.y as u32) < GROUND_SIZE)
        && ((position.y as u32) > 0) {
        position.x = x as f32;
        position.y = y as f32;
        //println!("монстр ходит кругами: x{} y{}", position.x, position.y);
    }
}

// По кругу должен ходить.
fn next_step_around(position: &mut Position, delta: f32, monster_state: &mut MonsterState) {
    //println!("монстр ходит кругами: x{} y{}",
    //       monster_state.target_point.x,
    //     monster_state.target_point.y);

    // проверяем достижение цели
    if (
        (position.x as u32 == monster_state.target_point.x as u32)
            && (position.y as u32 == monster_state.target_point.y as u32)
    )
        ||
        (
            (position.x - monster_state.target_point.x).abs() > 10f32 ||
                (position.y - monster_state.target_point.y).abs() > 10f32
        ) {
        // цель достигнута, ставим новую цель:
        // выбираем новую цель по направлению монстра.
        // две клетки вперед и одну вправо. меняем направление монстра.
        match monster_state.move_target.direct {
            Direction::North | Direction::NorthWest => {
                if (position.x as u32) < GROUND_SIZE - 4 { monster_state.target_point.x = position.x + 4f32; };
                if (position.y as u32) > 1 { monster_state.target_point.y = position.y - 1f32; };
                monster_state.move_target.direct = Direction::West;
            }

            Direction::West | Direction::WestSouth => {
                if (position.y as u32) > 5 { monster_state.target_point.y = position.y - 5f32; };
                if (position.x as u32) > 2 { monster_state.target_point.x = position.x - 2f32; };
                monster_state.move_target.direct = Direction::South;
            }

            Direction::South | Direction::SouthEast => {
                if (position.x as u32) > 6 { monster_state.target_point.x = position.x - 6f32; };
                if (position.y as u32) < GROUND_SIZE - 2 { monster_state.target_point.y = position.y + 2f32; };
                monster_state.move_target.direct = Direction::East;
            }

            Direction::East | Direction::EastNorth => {
                if (position.y as u32) < GROUND_SIZE - 7 { monster_state.target_point.y = position.y + 7f32; };
                if (position.x as u32) < GROUND_SIZE - 3 { monster_state.target_point.x = position.x + 3f32; };
                monster_state.move_target.direct = Direction::North;
            }
        }
        //println!("NEW target_x {}, target_y {}", monster_state.target_point.x, monster_state.target_point.y);
    } else {
        // цель не достигнута, делаем шаг в сторону цели.
        next_step_make(position, monster_state.target_point, monster_state, delta);
    }
}


/// Монстр идет к другому монстру
fn move_to_lead(entity: &Entity,
                mut position: &mut Position,
                mut monster_state: &mut MonsterState,
                monster_id: &MonsterId,
                speed: f32) -> Status {
    println!("Монстр {} идет к ВОЖДЮ", monster_id.id);
    let delta = speed;
    // проверяем достижение цели
    if let Some(lead_pos) = monster_state.lead_point {
        if (position.x as u32, position.y as u32) == (lead_pos.x as u32, lead_pos.y as u32) {
            // цель достигнута,
            //println!("монстр {} добрался до цели", monster_id.id);
            monster_state.find_around_count = 0;
            return Status::Success;
        } else {
            // цель не достигнута, делаем шаг в сторону цели.
            next_step_make(&mut position, lead_pos, &mut monster_state, delta);
        }

        entity.add_component(Replication); // произошли изменения монстра.
        entity.refresh();

        Status::Running
    } else {
        Status::Failure
    }
}
