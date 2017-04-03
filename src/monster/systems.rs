// будем юзать Behaviour Tree AI

//Главный скрипт работает по системе выбор-условие-действие (selector-condition-action).

//Выбор — своеобразный аналог оператора switch() в языках программирования.
// В «теле» элемента выбора происходит выбор одного из заданных наборов действий
// в зависимости от условия.

//Условие — проверка истинности заданного условия.
// Используется в начале каждого набора действий внутри элемента выбора.
// Если условие истинно — выполняется данный набор действий и выбор завершается.
// Если нет — происходит переход к следующему набору действий

//Действие — скрипт, запускающий другой скрипт (действие) с заданными параметрами.
// *В BT AI существует понятие базовых действий.

/*
SelectorBegin('AI Role 1');
    SequenceBegin('Атака');
        //видим врага и знаем его id
        Condition(SeeEnemy && Enemy>0);
        //смомтрим на него
        Action( Action_LookAt, point_direction(x,y, Enemy.x, Enemy.y));
        //стреляем в сторону врага 2 раза
        Action( Action_Shoot, point_direction(x,y, Enemy.x, Enemy.y), 2);
        SelectorBegin('Подходим на оптимальное растояние');
            //или
            SequenceBegin('Враг слишком далеко');
                Condition(point_distance(x,y, Enemy.x, Enemy.y)>256);
                Action(Action_MoveTo, Enemy.x-lengthdir_x(128, direction), Enemy.y-lengthdir_y(128, direction), highSpeed);
            SequenceEnd();
            //или
            SequenceBegin('Враг слишком близко');
                Condition(point_distance(x,y, Enemy.x, Enemy.y)<64);
                //идем назад
                Action(Action_MoveTo, x-lengthdir_x(64, direction), y-lengthdir_y(64, direction), highSpeed);
            SequenceEnd();
            SequenceBegin('маневр');
                //иначе просто маневрируем, чтобы сложнее было попасть
                Action( Action_MoveTo, x+irandom_range(-64, 64), y+irandom_range(-64, 64), highSpeed);
            SequenceEnd();
        SelectorEnd();
        //стреляем в сторону врага 4 раза
        Action(Action_Shoot, point_direction(x,y, Enemy.x, Enemy.y), 2);
    SequenceEnd();
SelectorEnd();
*/

//Selector — оператор выбора набора действий
//Sequence — набор действий
//Condition — проверка условия
//Action — действие. вызов скрипта(первый аргумент) с параметрами (остальные аргументы)

/*
http://www.pvsm.ru/robototehnika/161885/print/
Узлы BT называют [10] задачами или поведениями. Каждая задача может иметь четыре состояния:

    «Успех», если задача выполнена успешно;
    - выкинуть нахер, заменитьт ошибкой. «Неудача», если условие не выполнено или задача, по какой-то причине, невыполнима;
    «В работе», если задача запущена в работу и ожидает завершения
    «Ошибка», если в программе возникает неизвестная ошибка.

 Результат работы любого узла всегда передается родительскому узлу, расположенному на уровень выше.
 Дерево просматривается с самого верхнего узла – корня. От него производится поиск в глубину начиная
 с левой ветви дерева. Если у одного узла есть несколько подзадач, они исполняются слева направо.

    Среди узлов выделяют следующие типы:
    -действие (action),
    -узел исполнения последовательности (sequence),
    -параллельный узел (parallel),
    -селектор (selector),
    -условие (condition),
    -инвертор (inverter).

 Действие представляет собой запись переменных или какое-либо движение.
 Узлы последовательностей поочередно исполняют поведения каждого дочернего узла до тех пор,
 пока один из них не выдаст значение «Неудача», «В работе» или «Ошибка».
 Если этого не произошло, возвращает значение «Успех».

 Узлы параллельных действий исполняют поведения дочерних узлов до тех пор,
 пока заданное количество из них не вернет статусы «Неудача» или «Успех».

 Селекторы поочередно исполняют поведения каждого дочернего узла до тех пор,
 пока один из них не выдаст значение «Успех», «В работе» или «Ошибка».
 Если этого не произошло, возвращает значение «Неудача».

 Условия содержат критерий, по которому определяется исход, и переменную.
 Например, условие «Есть ли в этой комнате человек?» перебирает все объекты в комнате
 и сравнивает их с переменной «Человек».

 Узлы инверсии выполняют функцию оператора NOT.
*/

/*
Основные типы узлов в дереве поведения:
    Action Node (действие)
    Просто некоторая функция, которая должна выполниться при посещении данного узла.

    Condition (условие)
    Обычно служит для того, чтобы определить, выполнять или нет следующие за ним узлы. При true вернет Success, а при false возвращает Fail.

    Sequencer (последовательность)
    Выполняет все вложенные узлы по порядку, пока какой-либо из них не завершится неудачей (в таком случае возвращает Fail), либо пока все они успешно не завершатся (тогда возвращает Success).

    Selector (селектор)
    В отличие от Sequencer, прекращает обработку, как только любой вложенный узел вернет Success.

    Iterator (итератор — выполняет роль цикла for)
    Используется для выполнения в цикле серии действий некоторое число раз.

    Parallel Node
    Выполняет все свои дочерние узлы «одновременно». Здесь не имеется ввиду, что узлы выполняются несколькими потоками. Просто создается иллюзия параллельного выполнения, аналогично корутинам в Unity3d.
    http://www.pvsm.ru/arhitektura-prilozhenij/88642

*/
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
                        monster_state.view_food = true;
                        break 'outer;
                    } else { monster_state.view_food = false; }
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
            let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
            // TODO возможно, стоит выдавать несколько событий одновременно(Vec[BehaviourEvent])
            // реакция на обнаружение еды.
            if monster_state.view_food
                // переключаем событие на Обнаружена еда.
                && !behaviour_event.event.contains(&BehaviorEventEnum::FoundFood) {
                //behaviour_event.event.iter().any(|ev| *ev != BehaviorEventEnum::FoundFood)
                // если в векторе behaviour_events.event, нет события FoundFood
                // то, добавляем его туда.
                behaviour_event.event.push(BehaviorEventEnum::FoundFood);
                println!("Новое событие: монстр {} обнаружил еду!", monster_id.id);
            };

            // реакция на голод.
            if monster_attr.hungry < monster_attr.danger_hungry && !monster_state.low_food
                // наступает событие - ГОЛОД
                // danger_hungry = 960
                // 959+, 960
                && !behaviour_event.event.contains(&BehaviorEventEnum::ComeHungry) {
                behaviour_event.event.push(BehaviorEventEnum::ComeHungry);
                monster_state.low_food = true;
                println!("Новое событие: монстр {} голоден!", monster_id.id);
            }

            //реакция на усталость.
            if monster_attr.power < monster_attr.danger_power && !monster_state.low_power
                // наступает событие - УСТАЛ
                && !behaviour_event.event.contains(&BehaviorEventEnum::ComeTired) {
                behaviour_event.event.push(BehaviorEventEnum::ComeTired);
                monster_state.low_power = true;
                println!("Новое событие: монстр {} устал!", monster_id.id);
            }


            // реакция на наелся
            if monster_state.low_food && (monster_attr.hungry > 990)
                // Монстр насытился
                && !behaviour_event.event.contains(&BehaviorEventEnum::EatFull) {
                behaviour_event.event.push(BehaviorEventEnum::EatFull);
                monster_state.low_food = false;
                println!("Новое событие: монстр {} сыт!", monster_id.id);
            }


            // реакция на отдохнул
            if monster_state.low_power && (monster_attr.power > 990)
                // Нет событий
                && !behaviour_event.event.contains(&BehaviorEventEnum::NoEvent) {
                behaviour_event.event.push(BehaviorEventEnum::NoEvent);
                monster_state.low_power = false;
                println!("Новое событие: монстр {} отдохнул!", monster_id.id);
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

    fn process_one(&mut self, entity: &mut Entity) {
        let mut monster_class = entity.get_component::<MonsterClass>();
        if monster_class.bios_time.to(PreciseTime::now()) > Duration::seconds(2 * MONSTER_SPEED) {
            let mut monster_attr = entity.get_component::<MonsterAttributes>();
            let behaviour_state = entity.get_component::<BehaviourEvents>(); // состояние
            let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
            // power
            if behaviour_state.action == BehaviorActions::Sleep {
                monster_attr.power += 1;
            } else {
                monster_attr.power -= monster_attr.speed;
            }
            // hungry
            if monster_attr.hungry > 0 {
                monster_attr.hungry -= 1;
            }

            println!("power {}, hungry {}, монстр {}", monster_attr.power, monster_attr.hungry, monster_id.id);
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
            if (position.x < position.y * 2f32) && (position.x > 0f32 + delta) {
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


/// Расчет следующего шага по кругу монстра.
// По кругу должен ходить.
pub fn next_step_around(position: &mut Position, delta: f32, monster_state: &mut MonsterState) {
    // проверяем достижение цели
    if (position.x as u32 == monster_state.target_point.x as u32)
        && (position.y as u32 == monster_state.target_point.y as u32) {
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
            //println!("Sequencer cursor {}", cursor);
            for (index, node_beh) in vec_sequence.iter_mut().enumerate().skip(*cursor) {
                //if index >= cursor { //.skip(cursor) пропускает все до cursor
                *cursor = index; // пометить узел на котором мы торчим
                let node_branch: &mut NodeBehavior = &mut (*node_beh);
                //println!("index {}", index);
                let status = exec_node(node_branch, entity, wind);
                // что делать если нам вернули что процесс еще выполняется?
                // прерываем и выходим, запоминая курсор.
                if status == Status::Running {
                    println!("return Sequencer cursor {}", cursor);
                    return status
                }
                //}
            }
            *cursor = 0; // успех или провал, обнуляем курсор, т.к. начинать полюбасу поновой)))
            //println!("return2 Sequencer cursor {}", cursor);
            return Status::Success
        }

        //
        BehaviorEnum::If(ref mut beh_enum1, ref mut beh_enum2, ref mut beh_enum3) => {
            //
            //println!("If cursor {}", cursor);
            if let BehaviorEnum::Action(beh_act1) = **beh_enum1 {
                let status = exec_action(beh_act1, entity, wind);
                match status {
                    Status::Success => {
                        if let BehaviorEnum::Action(beh_act2) = **beh_enum2 {
                            return exec_action(beh_act2, entity, wind);
                        }
                    }

                    Status::Failure => {
                        if let BehaviorEnum::Action(beh_act3) = **beh_enum3 {
                            return exec_action(beh_act3, entity, wind);
                        }
                    }

                    Status::Running => {
                        return Status::Running
                    }
                }
            };
        }

        _ => { return Status::Success }
    }


    // вернуть статус, если больше нет дочерних узлов.
    Status::Running
}

/*// Функциональный подход
let sum_of_squared_odd_numbers: u32 =
    (0..).map(|n| n * n)             // Все натуральные числа в квадрате
         .take_while(|&n| n < upper) // Ниже верхнего предела
         .filter(|n| is_odd(*n))     // Это нечётные
         .fold(0, |sum, i| sum + i); // Суммируем*/


/// Активация действий монстра
pub fn exec_action(action: BehaviorActions, entity: &Entity, wind: &WindDirection) -> Status {
    match action {
        // 0. ХЗ, резервёд Ё
        BehaviorActions::Init => run_init(entity),
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
pub fn run_init(entity: &Entity) -> Status {
    let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    println!("Init монстр {}", monster_id.id);
    Status::Success
}


/// Действие монстра "спать"
pub fn run_sleep(entity: &Entity) -> Status {
    let mut behaviour_state = entity.get_component::<BehaviourEvents>(); // состояние
    let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    behaviour_state.action = BehaviorActions::Sleep;
    println!("...zzz...монстр {}", monster_id.id);
    Status::Running
}


/// Действие монстра "проверить усталость"
pub fn run_check_tired(entity: &Entity) -> Status {
    let monster_attr = entity.get_component::<MonsterAttributes>(); // атрибуты/характеристики
    //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    //println!("монстр {} проверяет усталость", monster_id.id);
    if monster_attr.power < monster_attr.danger_power {
        println!("монстр устал");
        return Status::Success
    }
    Status::Failure
}


/// Действие монстра "проверить голод"
pub fn run_check_hungry(entity: &Entity) -> Status {
    let monster_attr = entity.get_component::<MonsterAttributes>(); // атрибуты/характеристики
    //let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    //println!("монстр {} проверяет голод", monster_id.id);
    if monster_attr.hungry < monster_attr.danger_hungry {
        println!("монстр голоден");
        return Status::Success
    }
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
    let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    let delta: f32 = monster_attr.speed as f32;
    // тут заставляем монстра ходить туда-сюда, бесцельно, куда подует)
    next_step(&mut position, delta, wind);
    behaviour_state.action = BehaviorActions::Walk;
    entity.add_component(Replication); // произошли изменения монстра.
    entity.refresh();
    println!("Монстр {} гуляет x:{}, y:{},", monster_id.id, position.x, position.y);
    Status::Success
}


/// Действие монстра "искать еду"
pub fn run_find_food(entity: &Entity) -> Status {
    let mut monster_attr = entity.get_component::<MonsterAttributes>(); // атрибуты/характеристики
    let mut monster_state = entity.get_component::<MonsterState>();
    let behaviour_event = entity.get_component::<BehaviourEvents>(); // события
    let mut position = entity.get_component::<Position>();
    let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    // поиск пищи. ходим по окружности
    if monster_attr.speed == 1 { monster_attr.speed = 2 };
    next_step_around(&mut position, monster_attr.speed as f32, &mut monster_state);
    //

    entity.add_component(Replication); // произошли изменения монстра.
    entity.refresh();

    // наелся
    if behaviour_event.event.contains(&BehaviorEventEnum::EatFull) {
        println!("монстр {} наелся", monster_id.id);
        return Status::Success
    }

    println!("Монстр {} ищет хавку x:{}, y:{},", monster_id.id, position.x, position.y);
    Status::Running
}


/// Действие монстра "трапезничать"
pub fn run_meal(entity: &Entity) -> Status {
    let mut monster_attr = entity.get_component::<MonsterAttributes>(); // атрибуты/характеристики
    let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
    monster_attr.hungry = 1000;
    println!("монстр {} поел", monster_id.id);
    Status::Success
}