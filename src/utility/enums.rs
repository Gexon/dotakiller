// Будем тут хранить все наши перечисления и функции к ним.
// спс Adaman'у, за подсказки по энумам.


/// Направление
pub enum Direction {
    North = 0,
    NorthWest = 1,
    West = 2,
    WestSouth = 3,
    South = 4,
    SouthEast = 5,
    East = 6,
    EastNorth = 7,
}

/// Действия монстра.
// тут будут действия что выполняет монстр.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Eq, Debug)]
pub enum BehaviorActions {
    // 0. ХЗ, резервёд Ё
    Init,
    // Проверить усталось.
    CheckTired,
    // Сон. Монстр ждет, в этот момент с ним ничего не происходит.
    Sleep,
    // Бодрствование. Случайное перемещение по полигону.
    Walk,
    // Проверить голоден ли.
    CheckHungry,
    // Поиск пищи.(ХЗ, сложная хрень и долгая)
    FindFood,
    // Поиск воды.
    FindWater,
    // Прием пищи.
    Meal,
    // Прием воды.
    WaterIntake,
    // Перемещение к цели.
    MoveToTarget,
}

///// Текущее состояния монстра.
//// имеется ввиду дерево поведений, для отслеживания.
//#[derive(PartialEq, Copy, Clone)]
//pub enum BehaviorState<A> {
//    /// Выполняет действие.
//    ActionState(A, Option<S>),
//    /// Отслеживает последовательность действий.
//    // usize - видимо вроде курсора
//    SequenceState(Vec<Behavior<A>>, usize, Box<State<A, S>>),
//    /// Отслеживает узел `If`.
//    /// если статус `Running`, то в процессе выбора.
//    /// если `Success`, то он оценивает поведение как успешное.
//    /// если `Failure`, то он оценивает поведение как провальное.
//    IfState(Box<Behavior<A>>, Box<Behavior<A>>, Status, Box<State<A, S>>),
//    /// Отслеживает селектор)
//    // до первого узла возвращающего Success
//    SelectorState(Vec<Behavior<A>>),
//    /// Отслеживает цикл
//    //роль цикла
//    WhileState(Box<Behavior<A>>, Vec<Behavior<A>>),
//}

///// Приклеиваем метод по выемке перечисления по индексу
//impl BehaviorStateEnum {
//    pub fn from_index(idx: u32) -> Self {
//        match idx {
//            //0 => BehaviorStateEnum::Init,
//            1 => BehaviorStateEnum::Sleep,
//            2 => BehaviorStateEnum::Walk,
//            3 => BehaviorStateEnum::FindFood,
//            4 => BehaviorStateEnum::FindWater,
//            5 => BehaviorStateEnum::Meal,
//            6 => BehaviorStateEnum::WaterIntake,
//            7 => BehaviorStateEnum::MoveToTarget,
//            8 => BehaviorStateEnum::CheckMoveToTarget,
//            _ => BehaviorStateEnum::Init,
//        }
//    }
//}

/// Собятия монстра
//#[derive(PartialEq, Copy, Clone)]
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Eq, Debug)]
pub enum BehaviorEventEnum {
    // Обнаружена вода.
    _FoundWater,
    // Обнаружена еда.
    FoundFood,
    // Наступил голод.
    ComeHungry,
    // Наступила жажда.
    _ComeThirsty,
    // Утомился.
    ComeTired,
    // Монстр насытился.
    EatFull,
    // Монстр напился.
    _DrinkFull,
    // Нет событий.
    NoEvent,
}

//impl BehaviorEventEnum {
//    pub fn _from_enum(in_enum: BehaviorEventEnum) -> u32 {
//        match in_enum {
//            BehaviorEventEnum::Init => 0,
//            BehaviorEventEnum::_FoundWater => 1,
//            BehaviorEventEnum::FoundFood => 2,
//            BehaviorEventEnum::ComeHungry => 3,
//            BehaviorEventEnum::_ComeThirsty => 4,
//            BehaviorEventEnum::ComeTired => 5,
//            BehaviorEventEnum::EatFull => 6,
//            BehaviorEventEnum::_DrinkFull => 7,
//            BehaviorEventEnum::NoEvent => 8,
//        }
//    }
//}

/// Список узлов графа
pub enum BehaviorEnum {
    //функция, которая должна выполниться при посещении данного узла.
    Action(BehaviorActions),
    // тут все ясно
    If(Box<BehaviorEnum>, Box<BehaviorEnum>, Box<BehaviorEnum>),
    //последовательность, до первого узла Fail, либо выполняет все и возвращает Success
    Sequencer(Vec<NodeBehavior>),
    //до первого узла возвращающего Success
    _Selector(Vec<NodeBehavior>),
    //роль цикла
    _While(Box<BehaviorEnum>, Vec<BehaviorEnum>),
}

/// Узел графа
pub struct NodeBehavior {
    // функция, которая должна выполниться при посещении данного узла.
    pub behavior: BehaviorEnum,
    // курсор, указывает на выполняющийся дочерний узел.
    pub cursor: usize,
    // статус выполнения.
    pub status: Status,
}


///// Описание поведения.
//// https://github.com/PistonDevelopers/ai_behavior/blob/master/src/behavior.rs
//// добавить к узлам курсор и статус
//#[derive(Clone, RustcDecodable, RustcEncodable, PartialEq)]
//pub enum Behavior<A> {
//    /// Описания высоко-уровеневых действий.
//    /// Простое действие.
//    Action(A),
//    /// Преобразует `Success` в `Failure` и наоборот.
//    //Fail(Box<Behavior<A>>), // надо ли?
//    /// Игнорирует неудачи и возвращает "Success".
//    //AlwaysSucceed(Box<Behavior<A>>), // надо ли?
//    /// Выполняет последовательность, пока не встретит "Success".
//    /// Завершается неудачей, нет ни одного "Success".
//    Select(Vec<Behavior<A>>),
//    /// `If(condition, success, failure)`
//    If(Box<Behavior<A>>, Box<Behavior<A>>, Box<Behavior<A>>),
//    /// Выполняет последовательность, пока все "Success".
//    /// Последовательность прерывается и возвращает сбой, если встречается сбой.
//    /// The sequence succeeds if all the behavior succeeds.
//    /// Последовательность возвращает "Success", если все элементы отработали без сбоев.
//    Sequence(Vec<Behavior<A>>),
//    /// Loops while conditional behavior is running.
//    /// Выполняется цикл в то время как "условное" поведение работает.
//    ///
//    /// Succeeds if the conditional behavior succeeds.
//    /// Выполняется, если условное поведение преуспевает.
//    /// Fails if the conditional behavior fails,
//    /// or if any behavior in the loop body fails.
//    /// Вернет сбой, если условное поведение не удается,
//    /// или если какое-либо поведение в тело цикла не выполняется.
//    //While(Box<Behavior<A>>, Vec<Behavior<A>>),
//    /// Runs all behaviors in parallel until all succeeded.
//    /// Работает все поведение параллельно, пока все получилось.
//    ///
//    /// Succeeds if all behaviors succeed.
//    /// Успех, если все элементы успешны.
//    /// Fails is any behavior fails.
//    /// Ошибка, если хотябы один вернет ошибку.
//    //WhenAll(Vec<Behavior<A>>), // надо ли?
//    /// Runs all behaviors in parallel until one succeeds.
//    /// Работает все поведение параллельно, пока хотябы одно поведение удачно.
//    ///
//    /// Succeeds if one behavior succeeds.
//    /// В том случае, если одно поведение преуспевает.
//    /// Fails if all behaviors failed.
//    /// Вернет сбой, если все элемент ы вернут сбой.
//    //WhenAny(Vec<Behavior<A>>), // надо ли?
//    /// Runs all behaviors in parallel until all succeeds in sequence.
//    /// Работает все поведение параллельно, пока все удается в определенной последовательности.
//    ///
//    /// Succeeds if all behaviors succeed, but only if succeeding in sequence.
//    /// В том случае, если все варианты поведения, удается, но только если успех в определенной последовательности.
//    /// Fails if one behavior fails.
//    /// Вернет неудачу, если хотябы в одном элементе происходит сбой.
//    After(Vec<Behavior<A>>),
//    // надо ли?
//}


/// Результат работы узла дерева.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Eq, Debug)]
pub enum Status {
    /// Удачный результат выполнения действия или узла дерева.
    Success,
    /// Провал.
    Failure,
    /// В процессе.
    Running,
}

///// Аргументы возвращаемый действиями(Экшоном).
//pub struct ActionArgs<'a, E: 'a, A: 'a, S: 'a> {
//    /// The event/Событие.
//    pub event: &'a E,
//    /// Тип выполняемого действия.
//    pub action: &'a A,
//    /// Состояние выполняемого действия(хз что это значит).
//    pub state: &'a mut Option<S>,
//}