// описание компонент монстра

use tinyecs::*;
use time::{PreciseTime};

//use ::utility::map::Map;
use ::utility::enums::*;

/// Координаты на полигоне.
pub struct PositionM {
    pub x: f32,
    pub y: f32,
    pub direct: Direction,
}


/// Информация о цели(Еда, Жертва, Враг, Лидер)
pub struct ActionTarget {
    pub position: PositionM,
    pub target_type: TargetType,
}


/// метка принадлежности к классу монстров.
pub struct MonsterClass {
    // убрал эти говнотаймеры сюда из MonsterState
    // т.к. получение таймера перед всеми работами с сущностью
    // в части передачи entity в функцию и из этой функции снова в функцию,
    // затем при попытке прочитать MonsterState как mut
    // приводило к #сразукраш
    pub growth_time: PreciseTime,
    pub reproduction_time: PreciseTime,
    pub bios_time: PreciseTime,
    pub event_time: PreciseTime,
    pub behavior_time: PreciseTime,
    pub selector_time: PreciseTime,
    pub perception_time: PreciseTime,
}

impl Component for MonsterClass {}


/// уникальный номер монстра
pub struct MonsterId {
    pub id: i32,
}

impl Component for MonsterId {}


/// информация о состоянии монстра
pub struct MonsterState {
    pub state: i32,
    // emo
    pub emo_state: i32,
    // устал
    pub low_power: bool,
    // голоден
    pub low_food: bool,
    // увидел еду
    pub view_food: bool,

    pub dead: i32,
    // количество попыток поиска еды около себя.
    pub find_around_count: i32,
    pub move_target: PositionM,
    pub old_position: PositionM,
    pub target_point: PositionM,
    pub delta_x: i32,
    pub delta_y: i32,
}

impl Component for MonsterState {}

impl MonsterState {
    pub fn new() -> MonsterState {
        MonsterState {
            state: 1,
            emo_state: 0,
            low_power: false,
            low_food: false,
            view_food: false,
            dead: 0,
            move_target: PositionM { x: 0f32, y: 0f32, direct: Direction::North },
            old_position: PositionM { x: 0f32, y: 0f32, direct: Direction::North },
            target_point: PositionM { x: 0f32, y: 0f32, direct: Direction::North },
            find_around_count: 0i32,
            delta_x: 0,
            delta_y: -1,
        }
    }
}

/// характеристики монстра и его текущее состояние
pub struct MonsterAttributes {
    // скорость монстра
    pub speed: i32,
    // силы монстра
    pub power: i32,
    // сытость
    pub hungry: i32,
    // является ли вожаком
    pub lead: bool,
    // id вожака
    pub id_lead: i32,
    // флаг группы
    pub in_group: bool,
    // список членов стаи(id), только для вожака.
    pub group_members: Vec<i32>,
    // нижний уровень сил монстра
    pub danger_power: i32,
    // нижний уровень сытости монстра
    pub danger_hungry: i32,
}
impl MonsterAttributes {
    pub fn new() -> MonsterAttributes {
        MonsterAttributes {
            speed: 1,
            power: 1000,
            hungry: 900,
            lead: false,
            id_lead: -1,
            in_group: false,
            group_members: Vec::new(),
            danger_power: 960,
            danger_hungry: 960,
        }
    }
}

impl Component for MonsterAttributes {}


/// тут будем хранить все объекты на карте.
pub struct MonsterMaps {
    pub action_target: ActionTarget,
    pub last_eating: PositionM,
    //pub view_map: Map<u8>,
    //pub foods_map: Vec<ActionTarget>,
    //pub waters_map: Map<u8>,
}

impl Component for MonsterMaps {}

impl MonsterMaps {
    pub fn new() -> MonsterMaps {
        MonsterMaps {
            action_target: ActionTarget {
                position: PositionM {
                    x: 0f32,
                    y: 0f32,
                    direct: Direction::North,
                },
                target_type: TargetType::None,
            },
            last_eating: PositionM {
                x: -1f32,
                y: -1f32,
                direct: Direction::North,
            },
        }
    }
}


/// Сосотояние монстра, для Behaviour Tree
pub struct _BehaviourGlobalState {
    //    Всего 2 глобальных состояния:
    //    0.  Инициализация, ошибка.
    //    1.  Размеренное. Монстр спокоен.
    //    2.  Тревожное. Потеря здоровья.
    pub global_state: u32,
}

impl Component for _BehaviourGlobalState {}


/// События происходящие с монстром
pub struct BehaviourEvents {
    // возникшие события. Может быть несколько.
    pub event: Vec<BehaviorEventEnum>,
    // текщие действие или экшон
    pub action: BehaviorActions,
    // текущее состояние дерева. Здесь тоже может быть только одно состояние.
    //pub state: BehaviorState,
}

impl Component for BehaviourEvents {}

/// Программатор Selection tree
// тут храним граф.
pub struct SelectionTree {
    // это вектор с узлами графа. Содержат программу поведения.
    pub selector: NodeBehavior,
    // родительский узел хранит курсор на активный дочерний узел.
}

impl SelectionTree {
    pub fn new() -> SelectionTree {
        // возможно необходимо выставить ограничение на количество попыток выполнения текущей задачи,
        // чтоб предотвратить зацикливание.
        //
        // заполняем граф руками, в будущем загрузка из БД.
        let node_root: NodeBehavior =
        //корень
            NodeBehavior {
                // корневая нода, хранит последовательность
                behavior: BehaviorEnum::Sequencer(vec![
                    NodeBehavior {
                        // второй слой, нода выбора, проверка голода.
                        behavior: BehaviorEnum::If(
                            Box::new(NodeBehavior {
                                behavior: BehaviorEnum::Action(BehaviorActions::CheckHungry),
                                cursor: 0,
                            }),
                            Box::new(NodeBehavior {
                                // третий слой, нода выбора, проверка поиска еды.
                                behavior: BehaviorEnum::If(
                                    Box::new(NodeBehavior {
                                        behavior: BehaviorEnum::Action(BehaviorActions::FindFood),
                                        cursor: 0,
                                    }),
                                    Box::new(NodeBehavior {
                                        behavior: BehaviorEnum::Action(BehaviorActions::Meal),
                                        cursor: 0,
                                    }),
                                    Box::new(NodeBehavior {
                                        // четвертый слой, поиск в памяти места еды
                                        behavior: BehaviorEnum::If(
                                            Box::new(NodeBehavior {
                                                behavior: BehaviorEnum::Action(BehaviorActions::CheckMemMeal),
                                                cursor: 0,
                                            }),
                                            Box::new(NodeBehavior {
                                                behavior: BehaviorEnum::Action(BehaviorActions::MoveToTarget),
                                                cursor: 0,
                                            }),
                                            Box::new(NodeBehavior {
                                                behavior: BehaviorEnum::Action(BehaviorActions::Walk),
                                                cursor: 0,
                                            }),
                                        ),
                                        cursor: 0,
                                    }),
                                ),
                                cursor: 0,
                            }),
                            Box::new(NodeBehavior {
                                behavior: BehaviorEnum::Action(BehaviorActions::Walk),
                                cursor: 0,
                            })
                        ),
                        cursor: 0,
                    },
                    NodeBehavior {
                        behavior: BehaviorEnum::If(
                            Box::new(
                                NodeBehavior {
                                    behavior: BehaviorEnum::Action(BehaviorActions::CheckTired),
                                    cursor: 0,
                                }),
                            Box::new(
                                NodeBehavior {
                                    behavior: BehaviorEnum::Action(BehaviorActions::Sleep),
                                    cursor: 0,
                                }),
                            Box::new(
                                NodeBehavior {
                                    behavior: BehaviorEnum::Action(BehaviorActions::Walk),
                                    cursor: 0,
                                }),
                        ),
                        cursor: 0,
                    }
                ]),
                cursor: 0,
            };

        SelectionTree {
            selector: node_root,
        }
    }
}

impl Component for SelectionTree {}

/// Гены монстра.
// тут будем хранить поведение и характеристики монстра.
// в упрощенном виде, для возможности мутации и скрещивания.
// все плюсовые гены это активные, все минусовые пассивные.
pub struct _Genome {
    behaviour: Vec<i32>,
    body: Vec<i32>,
}

impl _Genome {
    pub fn _new() -> _Genome {
        // храним программу селектора. в будущем загрузка из БД
        // event, state
        let behavior = vec![0, 0];
        let body = vec![0, 0];
        _Genome {
            behaviour: behavior,
            body: body,
        }
    }
}

impl Component for _Genome {}