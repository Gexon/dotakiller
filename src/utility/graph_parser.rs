// парсим всякие графы полученные из БД

use ::utility::enums::*;

/// парсим граф монстра
pub fn monster_graph_parser(in_graph: &[(i32, i32)]) -> NodeBehavior {
    get_node(in_graph, 1)

    /*
            // заполняем граф руками, в будущем загрузка из БД.
            //корень
            NodeBehavior {
                // корневая нода, хранит последовательность
                behavior: NodeType::Sequencer(vec![
                    NodeBehavior {
                        // ветка голода. второй слой, нода выбора.
                        behavior: NodeType::If(
                            Box::new(NodeBehavior {
                                behavior: NodeType::Action(BehaviorActions::CheckHungry),
                                cursor: 0,
                            }),
                            Box::new(NodeBehavior {
                                // третий слой, нода выбора, проверка поиска еды.
                                behavior: NodeType::If(
                                    Box::new(NodeBehavior {
                                        behavior: NodeType::Action(BehaviorActions::FindFood),
                                        cursor: 0,
                                    }),
                                    Box::new(NodeBehavior {
                                        behavior: NodeType::Action(BehaviorActions::Meal),
                                        cursor: 0,
                                    }),
                                    Box::new(NodeBehavior {
                                        // четвертый слой, поиск в памяти места еды
                                        behavior: NodeType::If(
                                            Box::new(NodeBehavior {
                                                behavior: NodeType::Action(BehaviorActions::CheckMemMeal),
                                                cursor: 0,
                                            }),
                                            Box::new(NodeBehavior {
                                                behavior: NodeType::Action(BehaviorActions::MoveToTarget),
                                                cursor: 0,
                                            }),
                                            Box::new(NodeBehavior {
                                                behavior: NodeType::Action(BehaviorActions::Null),
                                                cursor: 0,
                                            }),
                                        ),
                                        cursor: 0,
                                    }),
                                ),
                                cursor: 0,
                            }),
                            Box::new(NodeBehavior {
                                behavior: NodeType::Action(BehaviorActions::Null),
                                cursor: 0,
                            })
                        ),
                        cursor: 0,
                    },
                    // ветка усталости
                    NodeBehavior {
                        behavior: NodeType::If(
                            Box::new(
                                NodeBehavior {
                                    behavior: NodeType::Action(BehaviorActions::CheckTired),
                                    cursor: 0,
                                }),
                            Box::new(
                                NodeBehavior {
                                    behavior: NodeType::Action(BehaviorActions::Sleep),
                                    cursor: 0,
                                }),
                            Box::new(
                                NodeBehavior {
                                    behavior: NodeType::Action(BehaviorActions::Null),
                                    cursor: 0,
                                }),
                        ),
                        cursor: 0,
                    },
                    // ветка ходьбы
                    NodeBehavior {
                        behavior: NodeType::Action(BehaviorActions::Walk),
                        cursor: 0,
                    },
                    // тут можно еще веток напихать
                ]),
                cursor: 0,
            }
            */
}

// получение очередного узла графа
// источник https://play.rust-lang.org/?gist=7f294e00df9b3bde61bf9ca1be6aa304&version=stable - собственная разработка.
pub fn get_node(edge_list: &[(i32, i32)], pos: i32) -> NodeBehavior {
    // всякие допы
    let mut seq_vec: Vec<NodeBehavior> = vec![];
    let mut if_vec: Vec<NodeBehavior> = vec![];
    // создаем пустой узел нашего графа
    let mut graph: NodeBehavior = NodeBehavior {
        behavior: NodeType::Sequencer(Vec::new()),
        cursor: pos as usize,
    };

    // кладем в узел следующие узлы
    match pos {
        1 => {
            // ищем ребра что выходят из нашего  узла
            for node in edge_list {
                if node.0 == pos {
                    // кладем в вектор наш следующий узел
                    seq_vec.push(get_node(edge_list, node.1));
                };
            };
            graph.behavior = NodeType::Sequencer(seq_vec);
        }
        2 | 3 | 6 | 10 | 15 => {
            // ищем три ребра что выходят из нашего IF  узла
            for node in edge_list {
                if node.0 == pos {
                    // кладем во временный  вектор узлы что выходят из нашего IF узла
                    if_vec.push(get_node(edge_list, node.1));
                };
            };

            if !if_vec.is_empty() {
                let node_beh1: NodeBehavior = if_vec.pop().unwrap();
                if !if_vec.is_empty() {
                    let node_beh2: NodeBehavior = if_vec.pop().unwrap();
                    if !if_vec.is_empty() {
                        let node_beh3: NodeBehavior = if_vec.pop().unwrap();
                        graph.behavior = NodeType::If(
                            Box::new(node_beh3),
                            Box::new(node_beh2),
                            Box::new(node_beh1),
                        );
                    } else {
                        graph.behavior = NodeType::If(
                            Box::new(node_beh2),
                            Box::new(node_beh1),
                            Box::new(get_null(pos)),
                        );
                    }
                } else {
                    graph.behavior = NodeType::If(
                        Box::new(node_beh1),
                        Box::new(get_null(pos)),
                        Box::new(get_null(pos)),
                    );
                }
            } else {
                graph.behavior = NodeType::If(
                    Box::new(get_null(pos)),
                    Box::new(get_null(pos)),
                    Box::new(get_null(pos)),
                );
            }
        }

        4 => {
            graph.behavior = NodeType::Action(BehaviorActions::Walk);
        }
        5 => {
            graph.behavior = NodeType::Action(BehaviorActions::CheckHungry);
        }
        8 => {
            graph.behavior = NodeType::Action(BehaviorActions::FindFood);
        }
        9 => {
            graph.behavior = NodeType::Action(BehaviorActions::Meal);
        }
        11 => {
            graph.behavior = NodeType::Action(BehaviorActions::CheckMemMeal);
        }
        12 => {
            graph.behavior = NodeType::Action(BehaviorActions::MoveToTarget);
        }
        13 => {
            graph.behavior = NodeType::Action(BehaviorActions::CheckTired);
        }
        14 => {
            graph.behavior = NodeType::Action(BehaviorActions::Sleep);
        }
        16 => {
            graph.behavior = NodeType::Action(BehaviorActions::CheckInGroup);
        }
        17 => {
            graph.behavior = NodeType::Action(BehaviorActions::JoinGroup);
        }

        // ставить только в конце
        _ => {
            graph.behavior = NodeType::Action(BehaviorActions::Null);
        }
    }

    // вертаем ответ
    graph
}

// возвращает заглушку NodeBehavior  Action
pub fn get_null(pos: i32) -> NodeBehavior {
    NodeBehavior {
        behavior: NodeType::Action(BehaviorActions::Null),
        cursor: pos as usize,
    }
}