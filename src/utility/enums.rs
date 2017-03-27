// Будем тут хранить все наши перечисления и функции к ним.
// спс Adaman'у, за подсказки по энумам.


/// Состояния монстра
#[derive(PartialEq, Copy, Clone)]
pub enum BehaviorStateEnum {
    //      0. Инициализация, ошибка.
    Init = 0,
    //      1. Сон. Монстр ждет, в этот момент с ним ничего не происходит.
    Sleep = 1,
    //      2. Бодрствование. Случайное перемещение по полигону.
    Walk = 2,
    //      3. Поиск пищи.
    FindFood = 3,
    //      4. Поиск воды.
    FindWater = 4,
    //      5. Прием пищи.
    Meal = 5,
    //      6. Прием воды.
    WaterIntake = 6,
    //      7. Перемещение к цели.
    MoveToTarget = 7,
    //      8. Проверка достижения цели.(?)
    CheckMoveToTarget = 8,
}

/// Приклеиваем метод по выемке перечисления по индексу
impl BehaviorStateEnum {
    pub fn from_index(idx: u32) -> Self {
        match idx {
            //0 => BehaviorStateEnum::Init,
            1 => BehaviorStateEnum::Sleep,
            2 => BehaviorStateEnum::Walk,
            3 => BehaviorStateEnum::FindFood,
            4 => BehaviorStateEnum::FindWater,
            5 => BehaviorStateEnum::Meal,
            6 => BehaviorStateEnum::WaterIntake,
            7 => BehaviorStateEnum::MoveToTarget,
            8 => BehaviorStateEnum::CheckMoveToTarget,
            _ => BehaviorStateEnum::Init,
        }
    }
}

/// Собятия монстра
#[derive(PartialEq, Copy, Clone)]
pub enum BehaviorEventEnum {
    // цифры обозначают приоритет события.
    //      0. Инициализация, ошибка.
    Init = 0,
    //      1. Обнаружена вода.
    _FoundWater = 1,
    //      2. Обнаружена еда.
    FoundFood = 2,
    //      3. Наступил голод.
    ComeHungry = 3,
    //      4. Наступила жажда.
    _ComeThirsty = 4,
    //      5. Утомился.
    ComeTired = 5,
    //      6. Монстр насытился.
    EatFull = 6,
    //      7. Монстр напился.
    _DrinkFull = 7,
    //      8. Нет событий.
    NoEvent = 8,
}

impl BehaviorEventEnum {
    pub fn from_enum(in_enum: BehaviorEventEnum) -> u32 {
        match in_enum {
            BehaviorEventEnum::Init => 0,
            BehaviorEventEnum::_FoundWater => 1,
            BehaviorEventEnum::FoundFood => 2,
            BehaviorEventEnum::ComeHungry => 3,
            BehaviorEventEnum::_ComeThirsty => 4,
            BehaviorEventEnum::ComeTired => 5,
            BehaviorEventEnum::EatFull => 6,
            BehaviorEventEnum::_DrinkFull => 7,
            BehaviorEventEnum::NoEvent => 8,
        }
    }
}

/// Список узлов графа
pub enum _NodeType {
    //функция, которая должна выполниться при посещении данного узла.
    Action,
    //чтобы определить, выполнять или нет следующие за ним узлы
    Condition,
    //последовательность, до первого узла Fail, либо выполняет все и возвращает Success
    Sequencer,
    //до первого узла возвращающего Success
    Selector,
    //роль цикла for
    Iterator,
    // пока не использую
    Parallel,
}


/// Описание поведения.
// https://github.com/PistonDevelopers/ai_behavior/blob/master/src/behavior.rs
#[derive(Clone, RustcDecodable, RustcEncodable, PartialEq)]
pub enum Behavior<A> {
    /// Описания высоко-уровеневых действий.
    Action(A),
    /// Преобразует `Success` в `Failure` и наоборот.
    //Fail(Box<Behavior<A>>), // надо ли?
    /// Игнорирует неудачи и возвращает "Success".
    //AlwaysSucceed(Box<Behavior<A>>), // надо ли?
    /// Выполняет последовательность, пока не встретит "Success".
    /// Завершается неудачей, нет ни одного "Success".
    Select(Vec<Behavior<A>>),
    /// `If(condition, success, failure)`
    If(Box<Behavior<A>>, Box<Behavior<A>>, Box<Behavior<A>>),
    /// Выполняет последовательность, пока все "Success".
    /// Последовательность прерывается и возвращает сбой, если встречается сбой.
    /// The sequence succeeds if all the behavior succeeds.
    /// Последовательность возвращает "Success", если все элементы отработали без сбоев.
    Sequence(Vec<Behavior<A>>),
    /// Loops while conditional behavior is running.
    /// Выполняется цикл в то время как "условное" поведение работает.
    ///
    /// Succeeds if the conditional behavior succeeds.
    /// Выполняется, если условное поведение преуспевает.
    /// Fails if the conditional behavior fails,
    /// or if any behavior in the loop body fails.
    /// Вернет сбой, если условное поведение не удается,
    /// или если какое-либо поведение в тело цикла не выполняется.
    While(Box<Behavior<A>>, Vec<Behavior<A>>),
    /// Runs all behaviors in parallel until all succeeded.
    /// Работает все поведение параллельно, пока все получилось.
    ///
    /// Succeeds if all behaviors succeed.
    /// Успех, если все элементы успешны.
    /// Fails is any behavior fails.
    /// Ошибка, если хотябы один вернет ошибку.
    //WhenAll(Vec<Behavior<A>>), // надо ли?
    /// Runs all behaviors in parallel until one succeeds.
    /// Работает все поведение параллельно, пока хотябы одно поведение удачно.
    ///
    /// Succeeds if one behavior succeeds.
    /// В том случае, если одно поведение преуспевает.
    /// Fails if all behaviors failed.
    /// Вернет сбой, если все элемент ы вернут сбой.
    //WhenAny(Vec<Behavior<A>>), // надо ли?
    /// Runs all behaviors in parallel until all succeeds in sequence.
    /// Работает все поведение параллельно, пока все удается в определенной последовательности.
    ///
    /// Succeeds if all behaviors succeed, but only if succeeding in sequence.
    /// В том случае, если все варианты поведения, удается, но только если успех в определенной последовательности.
    /// Fails if one behavior fails.
    /// Вернет неудачу, если хотябы в одном элементе происходит сбой.
    After(Vec<Behavior<A>>),
    // надо ли?
}


/// The result of a behavior or action.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Eq, Debug)]
pub enum Status {
    /// The behavior or action succeeded.
    Success,
    /// The behavior or action failed.
    Failure,
    /// The behavior or action is still running.
    Running,
}