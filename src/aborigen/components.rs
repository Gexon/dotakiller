// описание компонент аборигена

use tinyecs::*;


/// метка принадлежности к классу аборигена.
pub struct AborigenClass {
    pub growth_time: PreciseTime,
    pub reproduction_time: PreciseTime,
    pub bios_time: PreciseTime,
    pub event_time: PreciseTime,
    pub behavior_time: PreciseTime,
    pub selector_time: PreciseTime,
    pub perception_time: PreciseTime,
}

impl Component for AborigenClass {}

/// уникальный номер аборигена
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AborigenId {
    pub id: i32,
}

impl Component for AborigenId {}

/// информация о состоянии аборигена
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AborigenState {
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

impl AborigenState {
    pub fn new() -> AborigenState {
        AborigenState {
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

impl Component for AborigenState {}

/// характеристики аборигена и его текущее состояние
pub struct AborigenAttributes {
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

impl AborigenAttributes {
    pub fn new() -> AborigenAttributes {
        AborigenAttributes {
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

impl Component for AborigenAttributes {}

/// тут будем хранить все объекты на карте.
// память аборигена
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AborigenMem {
    pub action_target: ActionTarget,
    pub last_eating: PositionM,

}

impl AborigenMem {
    pub fn new() -> AborigenMem {
        AborigenMem {
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

impl Component for AborigenMem {}