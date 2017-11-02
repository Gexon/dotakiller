// парсим всякие графы полученные из БД

use ::utility::enums::*;

/// парсим граф монстра
pub fn monster_graph_parser(_in_graph: &[(i32, i32)]) -> NodeBehavior {
    // создаем пустой граф с одним пустым секвенсером.
    let graph: NodeBehavior;
    let mut max_edge1: i32 = 0;
    let mut cursor_edge1: i32 = 0;
    graph = NodeBehavior {
        behavior: BehaviorEnum::Sequencer(Vec!()),
        cursor: 0,
    };

    // сборка графа
    // граф собираем задом-наперед, т.к. в прямой последовательности будут сложности
    // с вложением одного узла в другой.
    // .
    // 1. берем ребро с наибольшим id исходного узла,
    // 2. берем номер узла из которого он выходит и создаем узел.
    // 3. помечаем содержимое ребра на входе значением 0, чтоб не обрабатывать его дважды.
    // - если этот узел Sequencer, то он безразмерный. кладем в него все узлы на которые указывают ребра.
    // - если это Action, то в него кладем только 1 узел с типом BehaviorActions.
    // - если это If, то в него влезет только три узла, берем первые три узла на которые указывают ребра.
    // вопрос: а что может быть больше узлов чем может влезть в узел, как такое возможно?
    // ответ: граф поведения передается по наследству и может быть искажен, что может привести к непредсказуем комбинациям.

    // реализация
    // 1. ищем максимальный узел из которого выходят стрелки
    let index: i32 = 0i32;
    for node in _in_graph {
        index += 1;
        if node.0 > max_edge1 {
            max_edge1 = node.0;
            cursor_edge1 = index;
        }
    }
    // 2. беерм номер узла из которого  он выходит и создаем узел.
    match _in_graph[cursor_edge1].0 {
        1 => {
            // обнуляем чтоб не парсить повторно в итерациях.
            _in_graph[cursor_edge1].0 = 0;
            // если это Sequencer, то нам нужен вектор из Vec<NodeBehavior>,
            // который мы положим в graph
            let seque_vec: Vec<NodeBehavior> = vec![];

            if _in_graph[cursor_edge1].0 == 1i32 {
                graph
            }
        }
        2,
        3,
        6,
        10 => {}
        _ => {}
    }


    // распечатка графа, для отладки.
    for node in _in_graph {
        println!("edge1 {}, edge2 {}", node.0, node.1);
        /*
         	  	1 	1 	01_Sequencer_Root
	  	  	  	2 	2 	02_If_Hungry
	  	  	  	3 	3 	03_If_Tired
	  	  	  	4 	4 	04_Action_Walk
	  	  	  	5 	5 	05_Action_CheckHungry
	  	  	  	6 	6 	06_If_FindFood
	  	  	  	7 	7 	07_Action_Null
	  	  	  	8 	8 	08_Action_FindFood
	  	  	  	9 	9 	09_Action_Meal
	  	  	  	10 	10 	10_If_CheckMemMeal
	  	  	  	11 	11 	11_Action_CheckMemMeal
	  	  	  	12 	12 	12_Action_MoveToTarget
	  	  	  	13 	13 	13_Action_CheckTired
	  	  	  	14 	14 	14_Action_Sleep
        */
        /*
        edge1 1, edge2 2
        edge1 1, edge2 3
        edge1 1, edge2 4
        edge1 2, edge2 5
        edge1 2, edge2 6
        edge1 2, edge2 7
        edge1 6, edge2 8
        edge1 6, edge2 9
        edge1 6, edge2 10
        edge1 10, edge2 11
        edge1 10, edge2 12
        edge1 10, edge2 7
        edge1 3, edge2 13
        edge1 3, edge2 14
        edge1 3, edge2 7
        */
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