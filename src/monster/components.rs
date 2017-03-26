// описание компонент монстра

use tinyecs::*;
use time::{PreciseTime};

use ::utility::map::Map;
use ::utility::enums::*;

/// Координаты на полигоне.
pub struct PositionM {
    pub x: f32,
    pub y: f32,
}

/// метка принадлежности к классу монстров.
pub struct MonsterClass;

impl Component for MonsterClass {}


/// уникальный номер монстра
pub struct MonsterId {
    pub id: i64,
}

impl Component for MonsterId {}


/// информация о состоянии монстра
pub struct MonsterState {
    pub state: i32,
    pub low_power: bool,
    pub low_food: bool,
    pub view_food: bool,

    pub growth_time: PreciseTime,
    pub reproduction_time: PreciseTime,
    pub bios_time: PreciseTime,
    pub event_time: PreciseTime,
    pub behavior_time: PreciseTime,
    pub selector_time: PreciseTime,
    pub perception_time: PreciseTime,
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
    pub view_map: Map<u8>,
    pub foods_map: Map<u8>,
    pub waters_map: Map<u8>,
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

/// Сосотояние монстра, для Behaviour Tree
pub struct BehaviourState {
    // и здесь тоже может быть только одно состояние.
    //  и кто-то должен его переключать =)
    pub state: BehaviorStateEnum,
}

impl Component for BehaviourState {}


/// Событий происходящее с монстром, для Behaviour Tree
pub struct BehaviourEvent {
    // видимо может быть только одно событие,
    // и обработчик событий loop event при отсутствии событий у монстра,
    // выставляет ему "6. Нет событий"
    pub event: BehaviorEventEnum,
}

impl Component for BehaviourEvent {}

/// Программатор Selection tree
// хранит код, связки, условия переключения состояний.
pub struct SelectionTree {
    // тут что-то типа кода алгоритма. правила обхода узлов
    // это вектор с массивами.
    pub selector: Vec<[u32; 2]>,
    //содержат программу поведения.
    pub curr_selector: i32,
    // предыдущая ячейка. храним индекс ячейки id

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
        // описание приоритета событий, для предотвращения зацикливания.
        // ComeHungry - 1(самый высокий приоритет), ComeTired - 2
        // пока узел не вернул выполнено или неудача, не запускать другие задачи.
        // возможно необходимо выставить ограничение на количество попыток выполнения текущей задачи,
        // чтоб предотвратить зацикливание.
        //
        // описание графа поведения при событии ComeTired(усталость)
        //
        // храним программу селектора. в будущем загрузка из БД
        //     [event, state], [event, state]
        let sel = vec![
            [BehaviorEventEnum::NoEvent as u32, BehaviorStateEnum::Walk as u32],
            [BehaviorEventEnum::ComeTired as u32, BehaviorStateEnum::Sleep as u32],
            [BehaviorEventEnum::ComeHungry as u32, BehaviorStateEnum::FindFood as u32],
            [BehaviorEventEnum::EatFull as u32, BehaviorStateEnum::Walk as u32]
        ]; // если событие 6, то переключить сосотояние на 2, с проверкой приоритетов и выполнения текущих задач.
        SelectionTree {
            selector: sel,
            curr_selector: -1,
            // текущий узел. -1 это инициализация либо ошибка.
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