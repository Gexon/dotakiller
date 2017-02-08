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

use tinyecs::*;
use time::{PreciseTime, Duration};

use MONSTER_SPEED;

use ::ground::components::Replication;
use ::ground::components::Position;
use ::ground::components::ClassGround;
use ::ground::components::WindDirection;
use ::monster::components::*;


/// Система восприятия
pub struct _PerceptionSystem;
// тут типа чекает окружение, и помечает объекты что попадают в поле зения.
impl System for _PerceptionSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass)
    }

    fn process_one(&mut self, _entity: &mut Entity) {
        // здесь тоже меняются события.

        // сканируем вокруг, может есть еда или вода или др. монстр или ОБОРИГЕН!
    }
}

/// Выбиральщик состояний дерева поведения
// используя код программатора SelectionTree, переключает состояния.
pub struct SelectorSystem;

impl System for SelectorSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(MonsterClass, SelectionTree, BehaviourEvent, BehaviourState)
    }

    //    fn process_no_entities(&mut self) {
    //        println!("instaced buffer render system must work, but no entities!");
    //    }
    //    fn process_no_data(&mut self) {
    //        println!("instaced buffer render system must work, but no data!");
    //    }

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
                    println!("ошибка/инициализация текущего указателя, теперь он {}", 0i32);
                } else {
                    /*event, state
                    let sel = vec![[6, 2], [5, 1]];*/
                    let index: usize = selection_tree.curr_selector as usize;
                    let curr_cell = selection_tree.selector[index]; //[6, 2]
                    let v_event = curr_cell[0];
                    let v_state = curr_cell[1];
                    // проверить нет ли ошибки в селекторе/программаторе. или первый запуск/инициализация.
                    let curr_event = behaviour_event.event; // считываем текущий событие/event
                    if curr_event == v_event {
                        // меняем состояние, на соответствующее.
                        behaviour_state.state = v_state;
                        println!("обнаружено событие {}", v_event);
                        println!("переключаю состояние на {}", v_state);
                        // сдвигаем curr_selector, переходим к сл. ячейке.
                        let shl: i32 = (len - 1) as i32;
                        if selection_tree.curr_selector < shl { selection_tree.curr_selector += 1; } else {
                            selection_tree.curr_selector = 0;
                        }
                    }
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
            let mut position = entity.get_component::<Position>();
            let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
            // сумоним ветер
            let ground = data.unwrap_entity();
            let wind = ground.get_component::<WindDirection>();
            //
            match behaviour_state.state {
                1 => {
                    println!("...zzz...монстр {}", monster_id.id);
                },
                2 => {
                    // тут заставляем монстра ходить туда-сюда, бесцельно, куда подует)
                    match wind.direction {
                        0 => {
                            if position.x < 140f32 {
                                position.x += 1f32;
                            }
                        },
                        1 => {
                            if position.x > position.y * 2f32 && position.x < 140f32 {
                                position.x += 1f32;
                            }
                        },
                        2 => {
                            if position.y > 0f32 {
                                position.y -= 1f32;
                            }
                        },
                        3 => {
                            if position.y < position.x * 2f32 && position.y > 0f32 {
                                position.y -= 1f32;
                            }
                        },
                        4 => {
                            if position.x > 0f32 {
                                position.x -= 1f32;
                            }
                        },
                        5 => {
                            if position.x < position.y * 2f32 && position.x > 0f32 {
                                position.x -= 1f32;
                            }
                        },
                        6 => {
                            if position.y < 140f32 {
                                position.y += 1f32;
                            }
                        },
                        7 => {
                            if position.y > position.x * 2f32 && position.y < 140f32 {
                                position.y += 1f32;
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
                _ => {},
            }
            // фиксируем текущее время
            monster_state.behavior_time = PreciseTime::now();
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
        //    1.  Обнаружена еда.
        //    2.  Обнаружена вода.
        //    3.  Наступил голод.
        //    4.  Наступила жажда.
        //    5.  Утомился.
        //    6.  Нет событий.
        //    7.  Монстр насытился.
        //    8.  Монстр напился.
        let mut monster_state = entity.get_component::<MonsterState>();
        if monster_state.event_time.to(PreciseTime::now()) > Duration::seconds(MONSTER_SPEED) {
            let mut behaviour_event = entity.get_component::<BehaviourEvent>(); // события
            let monster_attr = entity.get_component::<MonsterAttributes>(); // события
            let monster_id = entity.get_component::<MonsterId>(); // удалить. для отладки
            if behaviour_event.event == 0 {
                // проверяем ошибки/инициализация
                behaviour_event.event = 6;
                println!("ошибка/инициализация текущего события монстра {}, теперь он {}", monster_id.id, 6);
            } else if monster_attr.power < 960 && monster_state.event_last != 5 {
                behaviour_event.event = 5; // наступает событие - УСТАЛ
                monster_state.event_last = behaviour_event.event;
                println!("Новое событие: монстр {} устал.", monster_id.id);
            } else if monster_attr.power > 990 && monster_state.event_last != 6 {
                behaviour_event.event = 6;
                monster_state.event_last = behaviour_event.event;
                println!("Новое событие: монстр {} отдохнул.", monster_id.id);
            }

            // фиксируем текущее время
            monster_state.event_time = PreciseTime::now();
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
            if monster_attr.power > 0 {
                if behaviour_state.state == 1 {
                    monster_attr.power += 1;
                } else {
                    monster_attr.power -= 1;
                }
                println!("power {}, монстр {}", monster_attr.power, monster_id.id);
            }
            // фиксируем текущее время
            monster_state.bios_time = PreciseTime::now();
        }
    }
}

