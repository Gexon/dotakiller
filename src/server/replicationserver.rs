// A tiny async echo server with tokio-core
// bredov - автор кода) https://www.liveedu.tv
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

use SERVER_IP;

//use ::server::connection::Connection;
use ::server::components::ReplicationServerClass;
//use ::ground::components::*;
//use ::flora::components::FloraState;
//use ::flora::components::HerbId;
use ::flora::components::FloraClass;
//use ::monster::components::MonsterId;
use ::monster::components::MonsterClass;
//use ::monster::components::MonsterState;

//#[derive(Clone)]
//struct TcpStreamCloneable {
//    inner: Rc<RefCell<::tokio_core::net::TcpStream>>,
//}
//
//impl ::std::io::Write for TcpStreamCloneable {
//    fn write(&mut self, data: &[u8]) -> ::std::io::Result<usize> {
//        self.inner.borrow_mut().write(data)
//    }
//
//    fn flush(&mut self) -> ::std::io::Result<()> {
//        self.inner.borrow_mut().flush()
//    }
//}
//
//impl ::tokio_io::AsyncWrite for TcpStreamCloneable {
//    fn shutdown(&mut self) -> ::futures::Poll<(), ::std::io::Error> {
//        let mut inner = ::std::cell::RefMut::map(self.inner.borrow_mut(),
//                                                 |inner| inner as &mut ::tokio_io::AsyncWrite);
//        inner.shutdown()
//    }
//}


pub type Connections = HashMap<SocketAddr, mpsc::UnboundedSender<Result<Message, MessageError>>>;

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


pub struct ReplicationServer {
    // реактор обработчик событий.
    pub core: ::tokio_core::reactor::Core,
    // тут список клиентов tx.
    pub connections: Arc<Mutex<Connections>>,
    //pub connections: Arc<Mutex<HashMap<SocketAddr, mpsc::UnboundedSender<String>>>>,
    // список клиентов с каналом приема.
    //pub connections_recive: Rc<(RefCell<(HashMap<SocketAddr, mpsc::UnboundedSender<String>>)>)>,
    pub connections_recive: Arc<Mutex<HashMap<SocketAddr, mpsc::UnboundedSender<String>>>>,
    // список клиентов с меткой нового подключения
    pub connections_info: Arc<Mutex<HashMap<SocketAddr, Connect>>>,
}

impl ReplicationServer {
    pub fn run(&mut self) {
        // Привязываем сокет сервера
        let hostname: &str = SERVER_IP;
        let port: &str = "6655";
        let address = format!("{}:{}", hostname, port);
        let addr = address.parse().unwrap();

        // Создаем цикл обработки событий и TCP слушателя для приема соединений.
        //let mut core = Core::new().unwrap();
        let handle = self.core.handle();
        let socket = TcpListener::bind(&addr, &handle).unwrap();
        println!("Слушаю порт: {}", addr);

        // Это однопоточный сервер, поэтому мы можем просто использовать RC и RefCell в
        // HashMap хранилище всех известных соединений.
        // будем хранить тут список структуры подключения. для хранения данных о подключении.
        //let connections = Rc::new(RefCell::new(HashMap::new()));
        //let mut connections_info_locked = self.connections_info.lock().unwrap();
        //let mut connections_locked = self.connections.lock().unwrap();
        let connections = self.connections.clone();
        let connections_recive = self.connections_recive.clone();
        let connections_info = self.connections_info.clone();

        let srv = socket
            .incoming()
            .for_each(move |(stream, addr)| {
                println!("Входящее соединение: {}", addr);
                let framed = stream.framed(MessageCodec);
                let (writer, reader) = framed.split();
                // Then register our address with the stream to send data to us.
                // Создать канал для потока, который в других сокетов будет
                // послать нам сообщения. Затем зарегистрировать наш адрес с потоком отправить
                // данных в США. RX Receive Data (Принимаемые данные) TX Transmit Data
                let (tx, rx) = mpsc::unbounded::<Result<Message, MessageError>>();

                connections.lock().unwrap().insert(addr, tx);
                connections_info.lock().unwrap()
                    .insert(addr, Connect { is_new: true, token: 0, name: String::new() });

                // Определяем, что мы делаем для активного/текущего ввода/вывода.
                // То есть, читаем кучу строк из сокета и обрабатываем их
                // и в то же время мы также записываем любые строки из других сокетов.
                let connections_inner = connections.clone();
                let connections_info_inner = connections_info.clone();
                // let reader = BufReader::new(reader);

                // Model читает частями из сокета путем mapping-га в бесконечном цикле
                // для каждой строки из сокета. Этот "loop" завершается с ошибкой
                // после того как мы получили EOF на сокете.
                // let _iter = stream::iter(iter::repeat(()).map(Ok::<(), Error>));
                let connections_inner_1 = connections.clone();

                let socket_reader = reader
                    // В случае ошибки отправляем её "отправителю" (socket_writer)
                    // чтобы тот послал сообщение об ошибке пользователю.
                    // Поскольку обработка входящих сообщений асинхронна, нужно
                    // явно дождаться ответа от отправителя об успешной отправке
                    // данных пользователю и только тогда вернуть ошибку
                    // (при этом закроется соединение).
                    .or_else(move |error| {
                        let conn = {
                            let mut conns = connections_inner_1.lock().unwrap();
                            conns.get_mut(&addr).unwrap().clone()
                        };
                        let error_description = error.to_string();

                        let mut message = MessageError::from(error);
                        let (tx, rx) = futures::oneshot();

                        message.complete = Some(tx);

                        conn.send(Err(message)).then(|_|
                            // ждём ответа "отправителя" об успешной отправке данных.
                            // и возвращаем ошибку.
                            rx.then(|_| Err(Error::new(ErrorKind::Other, error_description))))
                    })
                    .for_each(move |message| {
                        println!("Message: {:?}", message);
                        match message {
                            message @ Message::Pos { .. } => {
                                let mut ci_locked = connections_info_inner.lock().unwrap();
                                if let Some(mut connect) = ci_locked.get_mut(&addr) {
                                    // let data = format!("{} {} {}", user, x, y);
                                    // TODO: commands::pos queries database. This should be
                                    // done in separate thread [futures_cpupool, r2d2]
                                    // let (smsg, return_token, return_name, _) = commands::pos(&data, &connect.token, connect.name.clone(), false);
                                    let message = comm::message_pos(&message, &mut connect, false);
                                    // todo не забыть прикрутить ресет
                                    // connect.token = return_token;
                                    // connect.name = return_name;

                                    let mut conns = connections_inner.lock().unwrap();
                                    let conns_iter = conns.iter_mut().filter(|&(&k, _)| k != addr).map(|(_, v)| v);

                                    for tx in conns_iter {
                                        if let Ok(message) = message.clone() {
                                            tx.send(Ok(message)).wait().unwrap();
                                        }
                                        //                                        match message.clone() {
                                        //                                            Ok(message) => tx.send(Ok(message)),
                                        //                                            Err(error) => {}, //Err(error) => tx.send(Err(MessageError::new(error))),
                                        //                                        }.wait().unwrap();
                                        // tx.send(Ok(Message::Raw(format!("{}: {}", addr, smsg)))).wait().unwrap();
                                    }

                                    Ok(())
                                } else {
                                    Err(Error::new(ErrorKind::Other, "Connection lost."))
                                }
                            }

                            Message::DelPos { .. } => Ok(()),

                            Message::UpdHerb { .. } => Ok(()),

                            Message::UpdMonster { .. } => Ok(()),

                            Message::Ping => Ok(()),

                            Message::Raw(message) => {
                                println!("raw message: {}", message);
                                Ok(())
                            }

                            Message::Error(..) => {
                                panic!("unreachable (due to or_else above");
                            }
                        }
                    })
                    .map_err(move |error| println!("[ERROR] ({}) {}", addr, error));
                // Всякий раз, когда мы получаем строку на приемнике, мы пишем его
                //`WriteHalf<TcpStream>`.
                // тут нужно собрать пакет для отправки.
                // склеить размер данных в первые 8 байт и сами данные.
                let socket_writer = rx.fold(writer, |writer, msg| match msg {
                    Ok(msg) => {
                        println!("[SEND] {:?}", msg);
                        writer.send(msg).map_err(|_| ()).boxed()
                    }
                    Err(error) => {
                        if let Some(tx) = error.complete {
                            writer
                                .send(Message::Error(error.inner.to_string()))
                                .map_err(|_| ())
                                .and_then(|writer| tx.send(()).and_then(move |_| Ok(writer)))
                                .boxed()
                        } else {
                            future::ok(writer).boxed()
                        }
                    }
                });

                // Теперь, когда мы получили futures, представляющие каждую половину гнезда, мы
                // используем `select` комбинатор ждать либо половину нужно сделать, чтобы
                // рушить другие. Тогда мы порождать результат.
                //let connections = self.connections.lock().unwrap();
                let connections = connections.clone();
                // let socket_reader = socket_reader.map_err(|_| ());
                let connection = socket_reader.map(|_| ()).select(socket_writer.map(|_| ()));
                handle.spawn(connection.then(move |_| {
                    // Для каждого открытого соединения, кроме отправителя, отправить
                    // строку через канал.
                    let mut conns = connections.lock().unwrap();
                    {
                        let iter = conns.iter_mut().filter(|&(&k, _)| k != addr).map(|(_, v)| v);
                        for tx in iter {
                            tx.send(Ok(Message::Raw(format!("Клиент {} отвалился.\n", addr))))
                                .wait()
                                .unwrap();
                        }
                    }
                    // ------------------------
                    conns.remove(&addr);
                    println!("Соединение {} закрыто.", addr);
                    Ok(())
                }));


                Ok(())
            });

        //        let srv = socket.incoming().for_each(move |(stream, addr)| {
        //            println!("Входящее соединение: {}", addr);
        //            let (reader, writer) = stream.split();
        //            // Then register our address with the stream to send data to us.
        //            // Создать канал для потока, который в других сокетов будет
        //            // послать нам сообщения. Затем зарегистрировать наш адрес с потоком отправить
        //            // данных в США. RX Receive Data (Принимаемые данные) TX Transmit Data
        //            let (tx, rx) = mpsc::unbounded::<String>();
        //
        //            {
        //                let mut connections_info_locked = connections_info.lock().unwrap();
        //                let mut connections_locked = connections.lock().unwrap();
        //                connections_locked.insert(addr, tx);
        //                connections_info_locked.insert(addr, Connect {
        //                    is_new: true,
        //                    token: 0i64,
        //                    name: "".to_string(),
        //                });
        //            }
        //
        //            // Определяем, что мы делаем для активного/текущего ввода/вывода.
        //            // То есть, читаем кучу строк из сокета и обрабатываем их
        //            // и в то же время мы также записываем любые строки из других сокетов.
        //            let connections_inner = connections.clone();
        //            let connections_inner2 = connections.clone();
        //            let reader = BufReader::new(reader);
        //            //let connection_info_clone = connections_info.clone();
        ////            let mut connections_info_locked = connections_info.lock().unwrap();
        ////            let ref mut connect = Connect {
        ////                token: 0i64,
        ////                name: "".to_string(),
        ////                is_new: true,
        ////            };
        ////            if let Some(mut in_connect) = connections_info_locked.get_mut(&addr) {
        ////                connect = in_connect;
        ////            }
        //            // Model читает частями из сокета путем mapping-га в бесконечном цикле
        //            // для каждой строки из сокета. Этот "loop" завершается с ошибкой
        //            // после того как мы получили EOF на сокете.
        //            let iter = stream::iter(iter::repeat(()).map(Ok::<(), Error>));
        //            let socket_reader = iter.fold(reader, move |reader, _| {
        //                // Прочитать строки из сокета, в противном случае, мы в EOF
        //                // читаем длину сообщения
        //                let buf = [0u8; 8];
        //                //let line = io::read_exact(reader, buf).and_then();
        //                let line = io::read_exact(reader, buf);
        //                let line = line.and_then(|(reader, buf)| {
        //                    // короче тут принимаем размер данных, создаем новый вектор для приема данных
        //                    // и передаем его следующей футуре. ппц наркомания токио.
        //                    let msg_len = BigEndian::read_u64(buf.as_ref());
        //                    if msg_len == 0 && msg_len > 1048576 {
        //                        Err(Error::new(ErrorKind::BrokenPipe, "Неверные размеры данных"))
        //                    } else {
        //                        //
        //                        let msg_len = msg_len as usize;
        //                        //debug!("Ожидаемая длина сообщения {}", msg_len);
        //                        let mut recv_buf: Vec<u8> = Vec::with_capacity(msg_len);
        //                        unsafe { recv_buf.set_len(msg_len); }
        //                        Ok((reader, recv_buf))
        //                    }
        //                });
        //
        //                let line = line.and_then(|(reader, buf_recive)| {
        //                    // читаем данные
        //                    io::read_exact(reader, buf_recive)
        //                });
        //
        //                let line = line.and_then(|(reader, buf)| {
        //                    //debug!("CONN : считано {} байт", n);
        //                    let mut recv2_buf = Vec::new();
        //                    let s = str::from_utf8(&buf[..]).unwrap();
        //                    let data = s.trim();
        //                    let data: Vec<&str> = data.splitn(2, ' ').collect();
        //                    match data[0] {
        //                        "pos" => {
        //                            let mut conns = connections_inner2.lock().unwrap();
        //                            if true {
        //                                // Для каждого открытого соединения, кроме отправителя, отправить
        //                                // строку через канал.
        //                                let iter = conns.iter_mut()
        //                                    .filter(|&(&k, _)| k != addr)
        //                                    .map(|(_, v)| v);
        //                                for tx in iter {
        //                                    tx.send(format!("{}", addr)).unwrap();
        //                                }
        //                                // берем записыватель и пишем.
        //                            } else {
        //                                let tx = conns.get_mut(&addr).unwrap();
        //                                tx.send("You didn't send valid UTF-8.".to_string()).unwrap();
        //                            }
        //
        //
        //                            //                            if let Some(mut in_connect) = connections_info_locked.get(&addr) {
        //                            //                                //connect = &mut in_connect;
        //                            //                            }
        //
        ////                            let (smsg, return_token, return_name, _return_reset) = comm::pos(
        ////                                data[1], &connect.token, &connect.name, false);
        ////                            // возвращаемые значения
        ////                            connect.token = return_token;
        ////                            connect.name = return_name;
        ////                            //self.is_reset = return_reset;
        ////                            // вектор с сообщением для рассылки.
        ////                            recv2_buf = smsg.into_bytes();
        ////                            let mut connections_recive_locked = connections_recive.lock().unwrap();
        ////                            if let Some(tx) = connections_recive_locked.get(&addr) {
        ////                                let message: String = data[1].to_string();
        ////                                tx.send(message).unwrap();
        ////                            }
        //                        }
        //                        //                        "ping" => {
        //                        //                            //let smsg: String = s.to_string();
        //                        //                            //let smsg_len = smsg.len();
        //                        //                            //recv2_buf = Vec::with_capacity(smsg_len);
        //                        //                            //unsafe { recv2_buf.set_len(smsg_len); }
        //                        //                            //recv2_buf = smsg.into_bytes();
        //                        //                            ()
        //                        //                        }
        //                        _ => { () }
        //                    }
        //                    Ok((reader, recv2_buf))
        //                });
        //
        //
        //                let line = line.and_then(|(reader, vec)| {
        //                    if vec.is_empty() {
        //                        Err(Error::new(ErrorKind::BrokenPipe, "broken pipe"))
        //                    } else {
        //                        Ok((reader, vec))
        //                    }
        //                });
        //
        //                let connections_inner = connections_inner.clone();
        //                // Преобразовать байты в строку, а затем отправить
        //                // строку всем остальным подключенным клиентам.
        //                let line = line.map(|(reader, vec)| {
        //                    (reader, String::from_utf8(vec))
        //                });
        //                //let connections = connections_inner.clone();
        //                line.map(move |(reader, message)| {
        //                    println!("{}: {:?}", addr, message);
        //                    let mut conns = connections_inner.lock().unwrap();
        //                    if let Ok(msg) = message {
        //                        // Для каждого открытого соединения, кроме отправителя, отправить
        //                        // строку через канал.
        //                        let iter = conns.iter_mut()
        //                            .filter(|&(&k, _)| k != addr)
        //                            .map(|(_, v)| v);
        //                        for tx in iter {
        //                            tx.send(format!("{}: {}", addr, msg)).unwrap();
        //                        }
        //                        // берем записыватель и пишем.
        //                    } else {
        //                        let tx = conns.get_mut(&addr).unwrap();
        //                        tx.send("You didn't send valid UTF-8.".to_string()).unwrap();
        //                    }
        //                    reader
        //                })
        //            });
        //
        //            // Всякий раз, когда мы получаем строку на приемнике, мы пишем его
        //            //`WriteHalf<TcpStream>`.
        //            // тут нужно собрать пакет для отправки.
        //            // склеить размер данных в первые 8 байт и сами данные.
        //            let socket_writer = rx.fold(writer, |writer, msg| {
        //                let amt = io::write_all(writer, msg.into_bytes());
        //                let amt = amt.map(|(writer, _)| writer);
        //                amt.map_err(|_| ())
        //            });
        //
        //            // Теперь, когда мы получили futures, представляющие каждую половину гнезда, мы
        //            // используем `select` комбинатор ждать либо половину нужно сделать, чтобы
        //            // рушить другие. Тогда мы порождать результат.
        //            //let connections = self.connections.lock().unwrap();
        //            let connections = connections.clone();
        //            let socket_reader = socket_reader.map_err(|_| ());
        //            let connection = socket_reader.map(|_| ()).select(socket_writer.map(|_| ()));
        //            handle.spawn(connection.then(move |_| {
        //                // Для каждого открытого соединения, кроме отправителя, отправить
        //                // строку через канал.
        //                let mut conns = connections.lock().unwrap();
        //                {
        //                    let iter = conns.iter_mut()
        //                        .filter(|&(&k, _)| k != addr)
        //                        .map(|(_, v)| v);
        //                    for tx in iter {
        //                        tx.send(format!("Клиент {} отвалился.\n", addr)).unwrap();
        //                    }
        //                }
        //                // ------------------------
        //                conns.remove(&addr);
        //                println!("Соединение {} закрыто.", addr);
        //                Ok(())
        //            }));
        //
        //
        //            Ok(())
        //        });

        //core.handle().spawn(srv.map_err(|_| ()));

        self.core.run(srv).unwrap();

        //        ReplicationServerSystem {
        //            core: core,
        //            //connections: connections_clone,
        //        }
    }
}

/*
    изменение типа future (map, map_err);
    запуск другого future, когда исходный будет выполнен (then, and_then, or_else);
    продолжение выполнения, когда хотя бы один из futures выполнился (select);
    ожидание выполнения двух future (join);
    определение поведения poll после вычислений (fuse).
*/