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


/// Виды целей для монстров.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Eq, Debug)]
pub enum TargetType{
    Flora,
    Monster,
    Aborigen,
    None,
}


/// Событие поедания растений монстрами
// делаем по координатам,
// для живых по id
pub struct EventEatFlora{
    pub value: i32,
    pub x: f32,
    pub y: f32,
}

/// Действия монстра.
// тут будут действия что выполняет монстр.
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Eq, Debug)]
pub enum BehaviorActions {
    // 0. ХЗ, резервёд Ё
    Null,
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





/// Собятия монстра
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Eq, Debug)]
pub enum BehaviorEventEnum {
    // Обнаружена вода.
    _FoundWater,
    // Обнаружена еда.
    FoundFood,
    // Наступил голод.
    BecomeHungry,
    // Наступила жажда.
    _BecomeThirsty,
    // Утомился.
    BecomeTired,
    // Монстр насытился.
    EatFull,
    // Монстр напился.
    _DrinkFull,
    // Монстр отдохнул.
    PowerFull,
    // Нет событий.
    NoEvent,
}



/// Список узлов графа
pub enum BehaviorEnum {
    //функция, которая должна выполниться при посещении данного узла.
    Action(BehaviorActions),
    // тут все ясно
    If(Box<NodeBehavior>, Box<NodeBehavior>, Box<NodeBehavior>),
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
    //pub status: Status,
}



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
