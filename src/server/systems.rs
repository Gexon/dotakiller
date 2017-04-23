// тут система от ECS, по репликации мира клиентам.

use tinyecs::*;


use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

use futures::sync::mpsc;

use ::server::proto::*;


use ::server::components::ReplicationServerClass;
use ::ground::components::*;
use ::flora::components::FloraState;
use ::flora::components::HerbId;
use ::flora::components::FloraClass;
use ::monster::components::MonsterId;
use ::monster::components::MonsterClass;
use ::monster::components::MonsterState;


pub struct ReplicationServerSystem {
    // тут список клиентов tx.
    pub connections: Arc<Mutex<Connections>>,
    // список клиентов с каналом приема.
    pub connections_recive: Arc<Mutex<HashMap<SocketAddr, mpsc::UnboundedSender<String>>>>,
    // список клиентов с меткой нового подключения
    pub connections_info: Arc<Mutex<HashMap<SocketAddr, Connect>>>,
}

impl System for ReplicationServerSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(ReplicationServerClass)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![FloraClass].optional(), aspect_all![MonsterClass].optional()]
    }

    // Вынимаем аспекты макросом, т.к. там безумие в коде.
    impl_process!(self, _entity, | _replication_server_class: ReplicationServerClass | with (_floras, _monsters) => {
        // Репликация растений ------------------------
        for flora in _floras {
            self.replication(|tx, _addr, conn| {
                // Выполняем первичную репликацию, если клиент новый.
                if conn.is_new {
                    let id_herb = flora.get_component::<HerbId>();
                    let class = flora.get_component::<Name>();
                    let position = flora.get_component::<Position>();
                    let state = flora.get_component::<FloraState>();
                    let s = format!("updherb {} {} {} {} {}", id_herb.id, class.name, state.state, position.x, position.y);
                    tx.send(Ok(Message::Raw(s))).unwrap();
                // основная репликация.
                } else if flora.has_component::<Replication>() {
                    let id_herb = flora.get_component::<HerbId>();
                    let class = flora.get_component::<Name>();
                    let position = flora.get_component::<Position>();
                    let state = flora.get_component::<FloraState>();
                    let s = format!("updherb {} {} {} {} {}", id_herb.id, class.name, state.state, position.x, position.y);
                    tx.send(Ok(Message::Raw(s))).unwrap();
                };
            });
            // Удалять флаг репликации тут! Иначе будет стопятьсот раз его пытаться удалить в каждом соединении.
            if flora.has_component::<Replication>() {
                flora.remove_component::<Replication>(); // убираем компонент репликации.
                flora.refresh();
            }
        } // --- -- Репликация растений

        // Репликация монстров ---------------------------
        for monster in _monsters {
            self.replication(|tx, _addr, conn| {
                // Выполняем первичную репликацию, если клиент новый.
                if conn.is_new {
                    let id_monstr = monster.get_component::< MonsterId > ();
                    let class = monster.get_component::< Name > ();
                    let state = monster.get_component::< MonsterState > ();
                    let position = monster.get_component::< Position > ();
                    let s = format ! ("updmonstr {} {} {} {} {}", id_monstr.id, class.name, state.state, position.x, position.y);
                    tx.send(Ok(Message::Raw(s))).unwrap();
                    // основная репликация.
                } else if monster.has_component::<Replication>() {
                    let id_monstr = monster.get_component::< MonsterId > ();
                    let class = monster.get_component::< Name > ();
                    let state = monster.get_component::< MonsterState > ();
                    let position = monster.get_component::< Position > ();
                    let s = format ! ("updmonstr {} {} {} {} {}", id_monstr.id, class.name, state.state, position.x, position.y);
                    tx.send(Ok(Message::Raw(s))).unwrap();
                }
            });
            // Удалять флаг репликации тут! Иначе будет стопятьсот раз его пытаться удалить в каждом соединении.
            if monster.has_component::<Replication>() {
                monster.remove_component::<Replication>(); // убираем компонент репликации.
                monster.refresh();
            }
        } // -------- Репликация монстров

        self.replication(|_tx, _addr, conn| {
            conn.is_new = false; // снимаем флаг новичка.
        });
    });
}

/// рассылка по всем клиентам.
impl ReplicationServerSystem {
    pub fn replication<F>(&mut self, mut f: F)
        where F: FnMut(&mpsc::UnboundedSender<Result<Message, MessageError>>,
            &SocketAddr,
            &mut Connect)
    {
        let mut connection_info_locked = self.connections_info.lock().unwrap();
        let connection_locked = self.connections.lock().unwrap();
        for (addr, mut conn) in connection_info_locked.iter_mut() {
            // рассылаем только новичкам
            //if conn.is_new {
            if let Some(tx) = connection_locked.get(addr) {
                f(tx, addr, &mut conn);
            }
            // переписать всем флаг новичка на старичка
            //conn.is_new = false;
            //}
        }
    }
}