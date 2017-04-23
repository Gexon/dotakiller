// тут система от ECS, по репликации мира клиентам.

use tinyecs::*;



use std::collections::HashMap;
use std::io::{Error, ErrorKind, Write, BufReader};
use std::sync::{Arc, Mutex};
use std::str::{self, FromStr};
use std::rc::Rc;
use std::cell::RefCell;
use std::iter;
use std::net::SocketAddr;

use futures::{self, future, Future, Sink, Poll};
use futures::stream::{self, Stream};
use futures::sync::oneshot::Sender;
use futures::sync::mpsc;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Decoder, Encoder};

use byteorder::{ByteOrder, BigEndian};
use bytes::{BytesMut, Buf, BufMut, IntoBuf, LittleEndian};

use ::server::commands as comm;
use ::server::proto::*;


//use ::server::connection::Connection;
use ::server::components::ReplicationServerClass;
//use ::ground::components::*;
//use ::flora::components::FloraState;
//use ::flora::components::HerbId;
use ::flora::components::FloraClass;
//use ::monster::components::MonsterId;
use ::monster::components::MonsterClass;
//use ::monster::components::MonsterState;


pub struct ReplicationServerSystem {
    // тут список клиентов tx.
    //pub connections: Arc<Mutex<HashMap<SocketAddr, mpsc::UnboundedSender<String>>>>,
    pub connections: Arc<Mutex<Connections>>,
    // список клиентов с каналом приема.
    //pub connections_recive: Rc<(RefCell<(HashMap<SocketAddr, mpsc::UnboundedSender<String>>)>)>,
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
        // обработка входящих соединений.
        //self.tick();

        // вектор векторов, для primary_replication. в нем храним все объекты с карты.
        let mut recv_obj: Vec<Vec<u8>> = Vec::new();
        // определяем наличие свежих подключений, тербующих primary_replication
        //let exist_new_conn = self.exist_new_conn();


//        if exist_new_conn {
//            // Выполняем первичную репликацию, на тот случай если объектов меньше 500.
//            println!("Обнаружено новое входящее соединение");
//            self.primary_replication(&mut recv_obj, true); // первичная репликация
//        }

        self.for_each_new_conn(|tx, _addr, _conn| {
            tx.send(Ok(Message::Raw("primary_replication\n".into()))).unwrap();
                    // tx.send(Ok("primary_replication\n".to_string()) /*addr,*//* message*/)
                    //     .unwrap();
                    // переписать всем флаг новичка на старичка
        })

    });
}


impl ReplicationServerSystem {
    pub fn for_each_new_conn<F>(&mut self, mut f: F)
        where F: FnMut(&mpsc::UnboundedSender<Result<Message, MessageError>>,
            &SocketAddr,
            &mut Connect)
    {
        let mut connection_info_locked = self.connections_info.lock().unwrap();
        let connection_locked = self.connections.lock().unwrap();
        for (addr, mut conn) in connection_info_locked.iter_mut() {
            // рассылаем только новичкам
            if conn.is_new {
                if let Some(tx) = connection_locked.get(addr) {
                    f(tx, addr, &mut conn);
                }

                conn.is_new = false;

                // if let Some(tx) = connection_locked.get(addr) {
                //     tx.send(Ok(Message::Raw(format!("primary_replication\n" /*addr,*//* message*/))))
                //         .unwrap();
                //     // tx.send(Ok("primary_replication\n".to_string()) /*addr,*//* message*/)
                //     //     .unwrap();
                //     // переписать всем флаг новичка на старичка
                //     conn.is_new = false;
                // }
            }
        }
    }

    /// оперделяем наличие новых соединений.
    // имеем проблему в получении списка новых клиентов =)
    pub fn _exist_new_conn(&mut self) -> bool {
        let connection_info_locked = self.connections_info.lock().unwrap();
        for (_addr, conn) in connection_info_locked.iter() {
            // вынуть из conn is_new
            if conn.is_new {
                return true
            }
        }
        false
    }

    // первичная репликация
    //    pub fn _primary_replication(&mut self, _recv_obj: &mut Vec<Vec<u8>>, _mark_old: bool) {
    //        //self.write_all_conn("primary_replication\n".to_string());
    //        // пометить всех кто принял первичную репликацию как стреньких.
    //        let mut connection_info_locked = self.connections_info.lock().unwrap();
    //        let connection_locked = self.connections.lock().unwrap();
    //        for (addr, mut conn) in connection_info_locked.iter_mut() {
    //            // рассылаем только новичкам
    //            if conn.is_new {
    //                if let Some(tx) = connection_locked.get(addr) {
    //                    //tx.send(format!("primary_replication\n" /*addr,*//* message*/)).unwrap();
    //                    tx.send("primary_replication\n".to_string() /*addr,*//* message*/).unwrap();
    //                    // переписать всем флаг новичка на старичка
    //                    conn.is_new = false;
    //                }
    //            }
    //        }
    //    }

    //    pub fn tick(&mut self) {
    //        // очередной тик сервака
    //        //self.core.turn(Some(Duration::new(0, 1)));
    //    }

    // Рассылаем всем подключениям
    // f_write.map(|_| ()).map_err(|_| ())
    //    pub fn _write_all_conn(&mut self, message: String) {
    //        let connection_locked = self.connections.lock().unwrap();
    //        let mut conns = connection_locked.clone();
    //        //if let Ok(msg) = message {
    //        // Для каждого открытого соединения, кроме отправителя, отправить
    //        // строку через канал.
    //        let iter = conns.iter_mut()
    //            //.filter(|&(&k, _)| k != addr)
    //            .map(|(_, v)| v);
    //        for tx in iter {
    //            tx.send(format!("write_all_conn: {}", /*addr,*/ message)).unwrap();
    //        }
    //    }
}