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
        aspect_all!(MonsterState)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround]]
    }

    fn process_d(&mut self, entity: &mut Entity, data: &mut DataList) {
        // здесь тоже меняются события.

        // сканируем вокруг, может есть еда или вода или др. монстр или ОБОРИГЕН!
        // сканируем с интервалом равным перемещению.
        let mut monster_state = entity.get_component::<MonsterState>();
        if monster_state.perception_time.to(PreciseTime::now()) > Duration::seconds(2 * MONSTER_SPEED) {
            // сканируем окружность на предмет кактусов.
            //_MonsterMaps
            let ground = data.unwrap_entity();
            let world_map = ground.get_component::<WorldMap>();
            // проверяем свободно ли место спавна.
            let position = entity.get_component::<Position>();
            let mut behaviour_event = entity.get_component::<BehaviourEvent>(); // события
            let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
            'outer: for x in -2..3 {
                for y in -2..3 {
                    let pos_x: i32 = (position.x.trunc() + x as f32) as i32;
                    let pos_y: i32 = (position.y.trunc() + y as f32) as i32;
                    let scan_point: Point = Point(pos_x, pos_y); // Casting
                    if !monster_state.view_food && world_map.flora[scan_point] == 1 {
                        // переключаем событие на 1-Обнаружена еда.
                        behaviour_event.event = BehaviorEventEnum::FoundFood; // наступает событие обнаружена еда
                        monster_state.view_food = true;
                        println!("Новое событие: монстр {} обнаружил еду!", monster_id.id);
                        break 'outer;
                    }
                }
            }


            // фиксируем текущее время
            monster_state.perception_time = PreciseTime::now();
        }
    }
}


/// Генерация событий
// Создаем события, проверяем параметры.
pub struct EventSystem;

impl System for EventSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterAttributes, BehaviourEvent)
    }

    fn process_one(&mut self, entity: &mut Entity) {
        //    0.  Инициализация, ошибка.
        //    1.  Обнаружена еда.+
        //    2.  Обнаружена вода.
        //    3.  Наступил голод.+
        //    4.  Наступила жажда.
        //    5.  Утомился.+
        //    6.  Нет событий.+
        //    7.  Монстр насытился.+
        //    8.  Монстр напился.
        let mut monster_state = entity.get_component::<MonsterState>();
        if monster_state.event_time.to(PreciseTime::now()) > Duration::seconds(MONSTER_SPEED) {
            let mut behaviour_event = entity.get_component::<BehaviourEvent>(); // события
            let monster_attr = entity.get_component::<MonsterAttributes>(); // события
            let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
            if behaviour_event.event == BehaviorEventEnum::Init {
                // проверяем ошибки/инициализация
                behaviour_event.event = BehaviorEventEnum::NoEvent;
                println!("ошибка/инициализация текущего события монстра {}, теперь он {}", monster_id.id, 6);
            } else if !monster_state.low_power && monster_attr.power < monster_attr.danger_power {
                behaviour_event.event = BehaviorEventEnum::ComeTired; // наступает событие - УСТАЛ
                monster_state.low_power = true;
                println!("Новое событие: монстр {} устал.", monster_id.id);
            } else if monster_state.low_power && monster_attr.power > 990 {
                behaviour_event.event = BehaviorEventEnum::NoEvent;
                monster_state.low_power = false;
                println!("Новое событие: монстр {} отдохнул.", monster_id.id);
            } else if !monster_state.low_food && monster_attr.hungry < monster_attr.danger_hungry {
                behaviour_event.event = BehaviorEventEnum::ComeHungry; // наступает событие - ГОЛОД
                monster_state.low_food = true;
                println!("Новое событие: монстр {} голоден.", monster_id.id);
            } else if monster_state.low_food && monster_attr.hungry > 990 {
                behaviour_event.event = BehaviorEventEnum::EatFull;
                monster_state.low_food = false;
                println!("Новое событие: монстр {} сыт.", monster_id.id);
            }


            // фиксируем текущее время
            monster_state.event_time = PreciseTime::now();
        }
    }
}


/// Выбиральщик состояний дерева поведения
// используя код программатора SelectionTree, переключает состояния.
pub struct SelectorSystem;

impl System for SelectorSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass, SelectionTree, BehaviourEvent, BehaviourState)
    }

    fn process_one(&mut self, entity: &mut Entity) {
        let mut monster_state = entity.get_component::<MonsterState>();
        if monster_state.selector_time.to(PreciseTime::now()) > Duration::seconds(MONSTER_SPEED) {
            let mut selection_tree = entity.get_component::<SelectionTree>();
            let mut behaviour_state = entity.get_component::<BehaviourState>(); // состояние
            let behaviour_event = entity.get_component::<BehaviourEvent>(); // события

            let len = selection_tree.selector.len();
            if len > 0 {
                // ткущий узел.
                if selection_tree.curr_selector < 0i32 {
                    selection_tree.curr_selector = 0i32;
                    println!("ошибка/инициализация текущего выбиральщика, теперь он {}", 0i32);
                } else {
                    // 1. Пока узел не вернул выполнено или неудача, не запускать другие задачи.
                    // 2. Соблюдать приоритет событий, для предотвращения зацикливания.(возможно прерывание менее приоритетной задачи).
                    // ComeHungry - 1(самый высокий приоритет), ComeTired - 2
                    // 3. Контролировать количество попыток выполнения текущей задачи, для предотвращения зацикливания.

                    println!("текущий выбиральщик {}", selection_tree.curr_selector);
                    /*event, state
                    let sel = vec![[6, 2], [5, 1], ...];*/
                    let index: usize = selection_tree.curr_selector as usize;
                    let curr_cell = selection_tree.selector[index]; //[6, 2]
                    let v_event = curr_cell[0];
                    let v_state = curr_cell[1];
                    if behaviour_event.event as u32 == v_event {
                        // меняем состояние, на соответствующее.
                        behaviour_state.state = BehaviorStateEnum::from_index(v_state);
                        println!("обнаружено событие {}", v_event);
                        println!("переключаю состояние на {}", v_state);
                    }
                }
                // сдвигаем curr_selector, переходим к сл. ячейке.
                let shl: i32 = (len - 1) as i32;
                if selection_tree.curr_selector < shl { selection_tree.curr_selector += 1; } else {
                    selection_tree.curr_selector = 0;
                }
            }


            // фиксируем текущее время
            monster_state.selector_time = PreciseTime::now();
        }
    }
}

/// Активатор. Приводит в действие.
// считывает состояние и выполняет его, либо продолжает выполнение.
// система поведения.
pub struct BehaviorSystem;

impl System for BehaviorSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass, BehaviourState, Position)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![ClassGround]]
    }

    fn process_d(&mut self, entity: &mut Entity, data: &mut DataList) {
        // смотрит текущее состояние и выполняет действие.
        // тут заставляем монстра ходить, пить, искать.
        //    0.  Инициализация, ошибка.
        //    1.  Сон. Монстр ждет, в этот момент с ним ничего не происходит.
        //    2.  Бодрствование. Случайное перемещение по полигону.
        //    3.  Поиск пищи.
        //    4.  Поиск воды.
        //    5.  Прием пищи.
        //    6.  Прием воды.
        //    7.  Перемещение к цели.
        //    8.  Проверка достижения цели.
        let mut monster_state = entity.get_component::<MonsterState>();
        if monster_state.behavior_time.to(PreciseTime::now()) > Duration::seconds(2 * MONSTER_SPEED) {
            let behaviour_state = entity.get_component::<BehaviourState>(); // состояние
            let mut monster_attr = entity.get_component::<MonsterAttributes>(); // атрибуты/характеристики
            let mut position = entity.get_component::<Position>();
            let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
            // сумоним ветер
            let ground = data.unwrap_entity();
            let wind = ground.get_component::<WindDirection>();
            //
            let delta: f32 = monster_attr.speed as f32;
            match behaviour_state.state {
                BehaviorStateEnum::Sleep => {
                    println!("...zzz...монстр {}", monster_id.id);
                },
                BehaviorStateEnum::Walk => {
                    // тут заставляем монстра ходить туда-сюда, бесцельно, куда подует)
                    match wind.direction {
                        0 => {
                            if position.x < 140f32 - delta {
                                position.x += delta;
                            }
                        },
                        1 => {
                            if position.x > position.y * 2f32 && position.x < 140f32 - delta {
                                position.x += delta;
                            }
                        },
                        2 => {
                            if position.y > 0f32 + delta {
                                position.y -= delta;
                            }
                        },
                        3 => {
                            if position.y < position.x * 2f32 && position.y > 0f32 + delta {
                                position.y -= delta;
                            }
                        },
                        4 => {
                            if position.x > 0f32 + delta {
                                position.x -= delta;
                            }
                        },
                        5 => {
                            if position.x < position.y * 2f32 && position.x > 0f32 + delta {
                                position.x -= delta;
                            }
                        },
                        6 => {
                            if position.y < 140f32 - delta {
                                position.y += delta;
                            }
                        },
                        7 => {
                            if position.y > position.x * 2f32 && position.y < 140f32 - delta {
                                position.y += delta;
                            }
                        },
                        _ => println!("странное направление ветра, вы не находите?"),
                    }
                    entity.add_component(Replication); // произошли изменения монстра.
                    entity.refresh();
                    println!("x:{}, y:{}, монстр {}", position.x, position.y, monster_id.id);
                    /*  движение по овальной траектории.
                        x = x_+x_radius*cos(r_ang+2);
                        y = y_+y_radius*sin(r_ang);

                        X1 = X + R * 0.5;
                        Y1 = Y + 1.3 * R * 0.8;
                        0.5 это синус 30
                        0.8 это косинус 30
                        R - хз. например 20 клеток.
                        X, Y - это текущие координаты.
                    */
                    //                    let x1: f32 = position.x + 20f32 * 0.5;
                    //                    let y1: f32 = position.y + 1.3f32 * 20f32 *0.8;
                    //                    position.x = x1;
                    //                    position.y = y1;
                    //                    println!("x:{}, y:{}", position.x, position.y);
                },
                BehaviorStateEnum::FindFood => {
                    // поиск пищи.
                    if monster_attr.speed == 1 { monster_attr.speed = 2 }
                    // перемещаемся по дуге, расставляя целевые точки пути.
                    //
                    /*
                    нужно сначала вычесть координаты точки V1, повернуть вектор V2, потом прибавить V1.
                    В общем это примерно так:
                    V3=V2-V1;
                    V2.x = V3.x*cos(a) - V3.y*sin(a);
                    V2.y = V3.x*sin(a) + V3.y*cos(a);
                    V2=V2+V1;
                    */
                    /*
                    https://habrahabr.ru/post/131931/
                    Чтобы сложить вектора, нам надо просто сложить каждую их составляющую друг с другом.
                    Например:
                    (0, 1, 4) + (3, -2, 5) = (0+3, 1-2, 4+5) = (3, -1, 9)

                    Длина вектора:
                    |V| = sqrt(x*x + y*y).

                    Нормализованный вектор:
                    Делим каждый компонент вектора на его длину.
                    V_n=(x/|V|, y/|V|)

                    Угол между нормализованными векторами:
                    Θ = acos(AB)
                    Пример:
                    Получим нормализованные вектора для (D') и для (V').
                    D' = D / |D| = (1, 1) / sqrt(1^2 + 1^2) = (1, 1) / sqrt(2) = (0.71, 0.71)
                    V' = V / |V| = (2, -1) / sqrt(2^2 + (-1)^2) = (2,-1) / sqrt(5) = (0.89, -0.45)
                    Затем определим угол между ними.
                    Θ = acos(D'V') = acos(0.71*0.89 + 0.71*(-0.45)) = acos(0.31) = 72


                    */
                    // рассчитываем целевую точку пути:
                    // вычисляем проекцию вектора, используя старые координаты и текущие.
                    let v_abx = position.x - monster_state.old_position.x;
                    let v_aby = position.y - monster_state.old_position.y;
                    // повернем проекцию вектора на 30 градусов.
                    // cos 30 grad = 0.866 rad, sin 30 = 0.5
                    let v_bx = v_abx * 0.866f32 - v_aby * 0.5f32;
                    let v_by = v_abx * 0.5f32 + v_aby * 0.866f32;
                    // накладываем проекцию вектора на текущие координаты.
                    monster_state.target_point.x = v_bx + position.x;
                    monster_state.target_point.y = v_by + position.y;
                    // сохраняем текущую точку пути, до перемещения в новое место, для последующих расчетов.
                    monster_state.old_position.x = position.x;
                    monster_state.old_position.y = position.y;
                    // рассчитываем следующий шаг, ближайший к целевой точке пути.
                    if position.x < monster_state.target_point.x {
                        if position.x < (140i32 - monster_attr.speed) as f32 {
                            position.x += monster_attr.speed as f32;// перемещаемся к этой точке по Х
                        }
                    } else if position.x > monster_state.target_point.x &&
                        position.x > monster_attr.speed as f32 {
                        position.x -= monster_attr.speed as f32;
                    }

                    if position.y < monster_state.target_point.y {
                        if position.y < (140i32 - monster_attr.speed) as f32 {
                            position.x += monster_attr.speed as f32;// перемещаемся к этой точке по У
                        }
                    } else if position.y > monster_state.target_point.y &&
                        position.y > monster_attr.speed as f32 {
                        position.y -= monster_attr.speed as f32;
                    }
                },
                _ => {},
            }
            // фиксируем текущее время
            monster_state.behavior_time = PreciseTime::now();
        }
    }
}


/// Bio Systems, будем умершвлять тут монстра.
// пересчет характеристик, значений жизнедеятельности монстра.
pub struct BioSystems;

impl System for BioSystems {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterAttributes, BehaviourState)
    }

    fn process_one(&mut self, entity: &mut Entity) {
        let mut monster_state = entity.get_component::<MonsterState>();
        if monster_state.bios_time.to(PreciseTime::now()) > Duration::seconds(2 * MONSTER_SPEED) {
            let mut monster_attr = entity.get_component::<MonsterAttributes>();
            let behaviour_state = entity.get_component::<BehaviourState>(); // состояние
            let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
            // power
            if behaviour_state.state as u32 == 1 {
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
            monster_state.bios_time = PreciseTime::now();
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