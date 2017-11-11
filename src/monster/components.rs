// описание компонент монстра

use tinyecs::*;
use time::PreciseTime;

use ::utility::enums::*;
use ::utility::db_query::get_monster_graph;
use ::utility::graph_parser::monster_graph_parser;

/// Координаты на полигоне.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PositionM {
    pub x: f32,
    pub y: f32,
    pub direct: Direction,
}


/// Информация о цели(Еда, Жертва, Враг, Лидер)
pub struct ActionTarget {
    pub position: PositionM,
    pub target_type: TargetType,
    pub target_id: i32,
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
    pub view_monster: bool,

    pub dead: i32,
    // количество попыток поиска еды около себя.
    pub find_around_count: i32,
    pub move_target: PositionM,
    pub old_position: PositionM,
    pub target_point: PositionM,
    pub lead_point: Option<PositionM>,
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
            view_monster: false,
            dead: 0,
            move_target: PositionM { x: 0f32, y: 0f32, direct: Direction::North },
            old_position: PositionM { x: 0f32, y: 0f32, direct: Direction::North },
            target_point: PositionM { x: 0f32, y: 0f32, direct: Direction::North },
            lead_point: None,
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
            danger_hungry: 960, // rand 800 // rand 450 // rand 700 каждое потомство рандомно.
            // шанс 10% 500, 9 из 10 потомков -> 500
        }
    }
}

impl Component for MonsterAttributes {}


/// тут будем хранить все объекты на карте.
// память монстра
pub struct MonsterMem {
    pub action_target: ActionTarget,
    pub last_eating: PositionM,

}

impl Component for MonsterMem {}

impl MonsterMem {
    pub fn new() -> MonsterMem {
        MonsterMem {
            action_target: ActionTarget {
                position: PositionM {
                    x: 0f32,
                    y: 0f32,
                    direct: Direction::North,
                },
                target_type: TargetType::None,
                target_id: -1i32,
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

        SelectionTree {
            selector: SelectionTree::get_graph(),
        }
    }

    //
    pub fn get_graph() -> NodeBehavior {
        // получаем граф с БД (список, хешмап, вектор, массив?)
        let graph = get_monster_graph();

        // передаем его в парсер
        monster_graph_parser(&graph)
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
        let behavior = vec![0, 0]; //
        //let behavior_slave = vec![0, 0]; // пассивные гены
        let _body = vec![0, 0];
        _Genome {
            behaviour: behavior,
            body: _body,
        }
    }
}

impl Component for _Genome {}