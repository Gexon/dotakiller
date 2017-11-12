// A tiny async echo server with tokio-core
// bredov - автор кода) https://www.liveedu.tv

use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::sync::{Arc, Mutex};
use std::str::{self};
use std::net::SocketAddr;
use time::PreciseTime;

use futures::{self, future, Future, Sink};
use futures::stream::Stream;
use futures::sync::mpsc;
use tokio_core::net::TcpListener;
use tokio_io::AsyncRead;

use ::server::commands as comm;
use ::server::proto::*;

use SERVER_IP;

pub struct ReplicationServer {
    // реактор обработчик событий.
    pub core: ::tokio_core::reactor::Core,
    // тут список клиентов tx.
    pub connections: Arc<Mutex<Connections>>,
    // список клиентов с каналом приема.
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
        let connections = Arc::clone(&self.connections);
        let connections_info = Arc::clone(&self.connections_info);

        let srv = socket
            .incoming()
            .for_each(move |(stream, addr)| {
                //println!("Входящее соединение: {}", addr);
                let framed = stream.framed(MessageCodec);
                let (writer, reader) = framed.split();
                // Then register our address with the stream to send data to us.
                // Создать канал для потока, который в других сокетов будет
                // послать нам сообщения. Затем зарегистрировать наш адрес с потоком отправить
                // данных в США. RX Receive Data (Принимаемые данные) TX Transmit Data
                let (tx, rx) = mpsc::unbounded::<Result<Message, MessageError>>();

                connections.lock().unwrap().insert(addr, tx);
                connections_info.lock().unwrap()
                    .insert(addr, Connect { is_new: true, token: 0, name: String::new(), primary_replication_time: PreciseTime::now(), });

                // Определяем, что мы делаем для активного/текущего ввода/вывода.
                // То есть, читаем кучу строк из сокета и обрабатываем их
                // и в то же время мы также записываем любые строки из других сокетов.
                let connections_inner = Arc::clone(&connections);
                let connections_info_inner = Arc::clone(&connections_info);

                // Model читает частями из сокета путем mapping-га в бесконечном цикле
                // для каждой строки из сокета. Этот "loop" завершается с ошибкой
                // после того как мы получили EOF на сокете.
                // let _iter = stream::iter(iter::repeat(()).map(Ok::<(), Error>));
                let connections_inner_1 = Arc::clone(&connections);

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
                        //println!("Message: {:?}", message);
                        match message {
                            message @ Message::Pos { .. } => {
                                let mut ci_locked = connections_info_inner.lock().unwrap();
                                if let Some(mut connect) = ci_locked.get_mut(&addr) {
                                    // let data = format!("{} {} {}", user, x, y);
                                    // TODO: commands::pos queries database. This should be
                                    // done in separate thread [futures_cpupool, r2d2]
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
                                    }

                                    Ok(())
                                } else {
                                    Err(Error::new(ErrorKind::Other, "Connection lost."))
                                }
                            }

                            Message::DelPos { .. }
                            | Message::UpdHerb { .. }
                            | Message::UpdMonster { .. }
                            | Message::Ping => Ok(()),

                            //Message::UpdHerb { .. } => Ok(()),

                            //Message::UpdMonster { .. } => Ok(()),

                            //Message::Ping => Ok(()),

                            Message::Raw(_message) => {
                                //println!("raw message: {}", message);
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
                //let socket_writer: BoxFuture
                // Огромное спасибо Alexander Irbis@alexander-irbis за помощь в отладке этого участка кода!
                let socket_writer
                = rx.fold(writer, |writer, msg|
                    match msg {
                        Ok(msg) => {
                            //println!("[SEND] {:?}", msg);
                            Box::new(writer.send(msg).map_err(drop))
                        }
                        Err(error) => {
                            // вынимает из исходящего канала все сообщения и если в очередном собщении ошибка то
                            if let Some(tx) = error.complete {
                                // то отправляет вместо сообщения, текст ошибки.
                                Box::new(writer
                                    .send(Message::Error(error.inner.to_string()))
                                    .map_err(drop)
                                    .and_then(|writer| tx.send(()).and_then(move |_| Ok(writer)))
                                ) as Box<Future<Item=_, Error=_>>
                            } else {
                                Box::new(future::ok(writer))
                            }
                        }
                    });

                // клонируем connections_info чтоб вынуть из него Client
                let connections_info_inner1 = Arc::clone(&connections_info);

                // Теперь, когда мы получили futures, представляющие каждую половину гнезда, мы
                // используем `select` комбинатор ждать либо половину нужно сделать, чтобы
                // рушить другие. Тогда мы порождать результат.
                let connections = Arc::clone(&connections);
                let connection = socket_reader.map(|_| ()).select(socket_writer.map(|_| ()));
                handle.spawn(connection.then(move |_| {
                    // Для каждого открытого соединения, кроме отправителя, отправить
                    // строку через канал.
                    let mut conns = connections.lock().unwrap();
                    {
                        let iter = conns.iter_mut().filter(|&(&k, _)| k != addr).map(|(_, v)| v);
                        for tx in iter {
                            let mut ci_locked = connections_info_inner1.lock().unwrap();
                            if let Some(connect) = ci_locked.get_mut(&addr) {
                                tx.send(Ok(Message::Raw(format!("delpos {}", connect.name))))
                                    .wait()
                                    .unwrap();
                            } else {
                                tx.send(Ok(Message::Raw(format!("Клиент {} отвалился.\n", addr))))
                                    .wait()
                                    .unwrap();
                            }
                        }
                    }
                    // ------------------------
                    conns.remove(&addr);
                    //println!("Соединение {} закрыто.", addr);
                    Ok(())
                }));


                Ok(())
            });

        self.core.run(srv).unwrap();
    }
}