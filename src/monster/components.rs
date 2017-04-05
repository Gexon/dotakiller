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
    pub id: i64,
}

impl Component for MonsterId {}


/// информация о состоянии монстра
pub struct MonsterState {
    pub state: i32,
    // устал
    pub low_power: bool,
    // голоден
    pub low_food: bool,
    // увидел еду
    pub view_food: bool,

    pub dead: i32,
    pub move_target: PositionM,
    pub old_position: PositionM,
    pub target_point: PositionM,
}

impl Component for MonsterState {}


/// характеристики монстра и его текущее состояние
pub struct MonsterAttributes {
    pub speed: i32,
    pub power: i32,
    pub hungry: i32,
    pub danger_power: i32,
    pub danger_hungry: i32,
}

impl Component for MonsterAttributes {}


/// тут будем хранить все объекты на карте.
pub struct _MonsterMaps {
    //pub view_map: Map<u8>,
    //pub foods_map: Map<u8>,
    //pub waters_map: Map<u8>,
}

impl Component for _MonsterMaps {}


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

    /*
    - Дерево ИИ переоценивает ИИ-ассоциированные Selection Variables.
    - Если выбранное поведение является тем же, что и текущее, то ничего не происходит.
    - Если выбранное поведение другое, тогда ИИ вызывает destructor текущего поведения.
    - ИИ переходит в новое поведение и...
    - Constructor нового поведения вызывается в ИИ.

    Selection Variables ассоциированные с ИИ модифицируются несколькими путями.

    ИИ система создает новые сигналы, которые передаются в систему поведения
    (например, система The Perception (система перцепции) определяет, что ИИ теперь может видеть игрока).
    ...Сигналы получаются BHS принадлежащем ИИ и обрабатываются как определено в секции Signal Variable.
    Текущее поведение ИИ получает сигнал и непосредственно изменяет Selection Variable.
    ИИ система сама непосредственно изменяет Selection Variable, принадлежащую ИИ
    (это обычно не требуется/в этом нет необходимости).
    */
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
                                status: Status::Running,
                            }),
                            Box::new(NodeBehavior {
                                // третий слой, нода выбора, проверка поиска еды.
                                behavior: BehaviorEnum::If(
                                    Box::new(NodeBehavior {
                                        behavior: BehaviorEnum::Action(BehaviorActions::FindFood),
                                        cursor: 0,
                                        status: Status::Running,
                                    }),
                                    Box::new(NodeBehavior {
                                        behavior: BehaviorEnum::Action(BehaviorActions::Meal),
                                        cursor: 0,
                                        status: Status::Running,
                                    }),
                                    Box::new(NodeBehavior {
                                        behavior: BehaviorEnum::Action(BehaviorActions::Init),
                                        cursor: 0,
                                        status: Status::Running,
                                    }),
                                ),
                                cursor: 0,
                                status: Status::Running,
                            }),
                            Box::new(NodeBehavior {
                                behavior: BehaviorEnum::Action(BehaviorActions::Walk),
                                cursor: 0,
                                status: Status::Running,
                            })
                        ),
                        cursor: 0,
                        status: Status::Running,
                    },
                    NodeBehavior {
                        behavior: BehaviorEnum::If(
                            Box::new(
                                NodeBehavior {
                                    behavior: BehaviorEnum::Action(BehaviorActions::CheckTired),
                                    cursor: 0,
                                    status: Status::Running,
                                }),
                            Box::new(
                                NodeBehavior {
                                    behavior: BehaviorEnum::Action(BehaviorActions::Sleep),
                                    cursor: 0,
                                    status: Status::Running,
                                }),
                            Box::new(
                                NodeBehavior {
                                    behavior: BehaviorEnum::Action(BehaviorActions::Walk),
                                    cursor: 0,
                                    status: Status::Running,
                                }),
                        ),
                        cursor: 0,
                        status: Status::Running,
                    }
                ]),
                cursor: 0,
                status: Status::Running,
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