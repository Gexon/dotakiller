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


/// Собятия монстра
#[derive(PartialEq, Copy, Clone)]
pub enum BehaviorEventEnum {
    //      0. Инициализация, ошибка.
    Init = 0,
    //      1. Обнаружена еда.
    FoundFood = 1,
    //      2. Обнаружена вода.
    _FoundWater = 2,
    //      3. Наступил голод.
    ComeHungry = 3,
    //      4. Наступила жажда.
    _ComeThirsty = 4,
    //      5. Утомился.
    ComeTired = 5,
    //      6. Нет событий.
    NoEvent = 6,
    //      7. Монстр насытился.
    EatFull = 7,
    //      8. Монстр напился.
    _DrinkFull = 8,
}


/// Вынимаем перечисление по индексу.
pub fn get_behavior_state_enum(idx: u32) -> BehaviorStateEnum {
    match idx {
        0 => BehaviorStateEnum::Init,
        1 => BehaviorStateEnum::Sleep,
        2 => BehaviorStateEnum::Walk,
        3 => BehaviorStateEnum::FindFood,
        4 => BehaviorStateEnum::FindWater,
        5 => BehaviorStateEnum::Meal,
        6 => BehaviorStateEnum::WaterIntake,
        7 => BehaviorStateEnum::MoveToTarget,
        8 => BehaviorStateEnum::CheckMoveToTarget,
        _ => { unreachable!() },
    }
}