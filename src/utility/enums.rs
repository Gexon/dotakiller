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


/// Типы событий для монстров при работе с группами
#[derive(PartialEq, Eq)]
pub enum EventTypeMonster {
    None = -1,
    RequestJoinGroup = 0, // События для ВОЖДЯ о вступлении в группу.(МОНСТР -> ВОЖДЮ)
    AnswerAcceptingGroup = 1, // Ответ от члена стаи о принятии приглоса в группу (МОНСТР -> ВОЖДЮ)
    MessageLeaveGroup = 2, // Событие от члена стаи для ВОЖДЯ о выходе из группы. (МОНСТР -> ВОЖДЮ)
    AnswerLeavingGroup = 3, // Ответ от ВОЖДЯ стаи для члена о его исключении из группы. (ВОЖДЬ -> МОНСТР)
    LeadLeaveGroup = 4, // Событие от ВОЖДЯ всем членам стаи о его выходе из стаи (ВОЖДЬ -> ВСЕМ)
}


/// Событие поедания растений монстрами
// делаем по координатам
pub struct EventEatFlora{
    pub value: i32,
    pub x: f32,
    pub y: f32,
}

/// Событие для взаимодействия монстров друг с другом
// для живых по id
pub struct EventGroupMonster{
    pub event_type: EventTypeMonster,
    pub id_sender: i32,
    pub id_receiver: i32,
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
    // Пошерудить в памяти о последнем месте с едой.
    CheckMemMeal,
    // Поиск монстров.
    FindMonsters,
    // Проверяем в группе ли монстр.
    CheckInGroup,
    // Вступаем в группу.
    JoinGroup,
}

/// Собятия монстра
#[derive(Copy, Clone, RustcDecodable, RustcEncodable, PartialEq, Eq, Debug)]
pub enum BehaviorEventEnum {
    // Нет событий.
    NoEvent,
    // Обнаружена вода.
    _FoundWater,
    // Обнаружена еда.
    FoundFood,
    // Обнаружен монстер.
    FoundMonster,
    // Наступил голод.
    BecomeHungry,
    // Наступила жажда.
    _BecomeThirsty,
    // Утомился.
    BecomeTired,
    // Треба еда
    NeedFood,
    // Треба группа
    NeedGroup,
    // Монстр насытился.
    EatFull,
    // Монстр напился.
    _DrinkFull,
    // Монстр отдохнул.
    PowerFull,
    // цель потеряна
    TargetLost,

}

/// Типы узлов графа
pub enum NodeType {
    //функция, которая должна выполниться при посещении данного узла.
    Action(BehaviorActions),
    // тут все ясно
    If(Box<NodeBehavior>, Box<NodeBehavior>, Box<NodeBehavior>),
    // последовательность, до первого узла Fail, либо выполняет все и возвращает Success
    Sequencer(Vec<NodeBehavior>),
    // до первого узла возвращающего Success
    _Selector(Vec<NodeBehavior>),
    // роль цикла
    _While(Box<NodeType>, Vec<NodeType>),
}

/// Узел графа
pub struct NodeBehavior {
    // функция, которая должна выполниться при посещении данного узла.
    pub behavior: NodeType,
    // курсор, указывает на выполняющийся дочерний узел.
    pub cursor: usize,
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
