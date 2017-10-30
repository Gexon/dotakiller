// парсим всякие графы полученные из БД

use ::utility::enums::*;

/// парсим граф монстра
pub fn monster_graph_parser(_in_graph: &[(i32, i32)]) -> NodeBehavior {
    // читаем так: my_vec[i].0, my_vec[i].1
    //
    for node in _in_graph {
        println!("edge1 {}, edge2 {}", node.0, node.1);
    }


    // заполняем граф руками, в будущем загрузка из БД.
    //корень
    NodeBehavior {
        // корневая нода, хранит последовательность
        behavior: BehaviorEnum::Sequencer(vec![
            NodeBehavior {
                // ветка голода. второй слой, нода выбора.
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
                                        behavior: BehaviorEnum::Action(BehaviorActions::Null),
                                        cursor: 0,
                                    }),
                                ),
                                cursor: 0,
                            }),
                        ),
                        cursor: 0,
                    }),
                    Box::new(NodeBehavior {
                        behavior: BehaviorEnum::Action(BehaviorActions::Null),
                        cursor: 0,
                    })
                ),
                cursor: 0,
            },
            // ветка усталости
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
                            behavior: BehaviorEnum::Action(BehaviorActions::Null),
                            cursor: 0,
                        }),
                ),
                cursor: 0,
            },
            // ветка ходьбы
            NodeBehavior {
                behavior: BehaviorEnum::Action(BehaviorActions::Walk),
                cursor: 0,
            },
            // тут можно еще веток напихать
        ]),
        cursor: 0,
    }
}