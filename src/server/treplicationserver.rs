// A tiny async echo server with tokio-core

use tinyecs::*;

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::iter;
use std::io::{Error, ErrorKind, BufReader};
//use std::time::Duration;
use std::net::SocketAddr;
//use std::thread;

use futures::Future;
use futures::stream::{self, Stream};
//use futures::executor::{self, Spawn, Unpark};
use futures::sync::mpsc;
use tokio_core::net::{TcpListener};
//use tokio_core::reactor::Core;
use tokio_io::*;
use tokio_io::AsyncRead;

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

#[derive(Clone)]
struct TcpStreamCloneable {
    inner: Rc<RefCell<::tokio_core::net::TcpStream>>,
}

impl ::std::io::Write for TcpStreamCloneable {
    fn write(&mut self, data: &[u8]) -> ::std::io::Result<usize> {
        self.inner.borrow_mut().write(data)
    }

    fn flush(&mut self) -> ::std::io::Result<()> {
        self.inner.borrow_mut().flush()
    }
}

impl ::tokio_io::AsyncWrite for TcpStreamCloneable {
    fn shutdown(&mut self) -> ::futures::Poll<(), ::std::io::Error> {
        let mut inner = ::std::cell::RefMut::map(self.inner.borrow_mut(),
                                                 |inner| inner as &mut ::tokio_io::AsyncWrite);
        inner.shutdown()
    }
}

pub struct Connect {
    //stream: TcpStreamCloneable,
    //pub tunnel_data: (mpsc::UnboundedSender<String>, mpsc::UnboundedReceiver<String>),
    //pub tunnel_sync: (mpsc::UnboundedSender<String>, mpsc::UnboundedReceiver<String>),
    pub is_new: bool,
}

pub struct ReplicationServerSystem {
    // тут список клиентов tx.
    pub connections: Rc<(RefCell<(HashMap<SocketAddr, mpsc::UnboundedSender<String>>)>)>,
    // список клиентов с каналом приема.
    pub connections_recive: Rc<(RefCell<(HashMap<SocketAddr, mpsc::UnboundedSender<String>>)>)>,
    // список клиентов с меткой нового подключения
    pub connections_info: Rc<(RefCell<(HashMap<SocketAddr, Connect>)>)>,
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
        self.tick();

        // вектор векторов, для primary_replication. в нем храним все объекты с карты.
        let mut recv_obj: Vec<Vec<u8>> = Vec::new();
        // определяем наличие свежих подключений, тербующих primary_replication
        let exist_new_conn = self.exist_new_conn();


        if exist_new_conn {
            // Выполняем первичную репликацию, на тот случай если объектов меньше 500.
            println!("Обнаружено новое входящее соединение");
            self.primary_replication(&mut recv_obj, true); // первичная репликация
        }

    });
}


impl ReplicationServerSystem {
    /// оперделяем наличие новых соединений.
    // имеем проблему в получении списка новых клиентов =)
    pub fn exist_new_conn(&mut self) -> bool {
        for (_addr, conn) in self.connections_info.borrow().iter() {
            // вынуть из conn is_new
            if conn.is_new {
                return true
            }
        }
        false
    }

    /// первичная репликация
    pub fn primary_replication(&mut self, _recv_obj: &mut Vec<Vec<u8>>, _mark_old: bool) {
        self.write_all_conn();
        //        for (addr, conn) in self.connections.borrow_mut().iter() {
        //            // переписать всем флаг новичка на старичка
        //            conn.is_new = false;
        //        }
        //        for c in self.conns.iter_mut() {
        //            if c.is_newbe() {
        //                while !recv_obj.is_empty() {
        //                    if let Some(message) = recv_obj.pop() {
        //                        let rc_message = Rc::new(message);
        //                        c.send_message(rc_message.clone())
        //                            .unwrap_or_else(|e| {
        //                                error!("Сбой записи сообщения в очередь {:?}: {:?}", c.token, e);
        //                                c.mark_reset();
        //                            });
        //                    }
        //                }
        //                if mark_old { c.mark_old(); }
        //            }
        //        }
    }

    pub fn tick(&mut self) {
        // очередной тик сервака
        //self.core.turn(Some(Duration::new(0, 1)));
    }

    /// Рассылаем всем подключениям
    // f_write.map(|_| ()).map_err(|_| ())
    pub fn write_all_conn(&mut self) {
        //        for (addr, conn) in self.connections.borrow().iter() {
        //            let f_write = io::write_all(conn.stream.clone(), "\n".as_bytes())
        //                .map(|_| ())//.and_then(|_| Ok(()))
        //                .map_err(|_| ());
        //            let handle = self.core.handle();
        //            handle.spawn(f_write);
        //        }
    }
}


pub struct ReplicationServer {
    // реактор обработчик событий.
    pub core: ::tokio_core::reactor::Core,
    // тут список клиентов tx.
    pub connections: Rc<(RefCell<(HashMap<SocketAddr, mpsc::UnboundedSender<String>>)>)>,
    // список клиентов с каналом приема.
    pub connections_recive: Rc<(RefCell<(HashMap<SocketAddr, mpsc::UnboundedSender<String>>)>)>,
    // список клиентов с меткой нового подключения
    pub connections_info: Rc<(RefCell<(HashMap<SocketAddr, Connect>)>)>,
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
        let connections = self.connections.clone();
        let connections_info = self.connections_info.clone();

        let srv = socket.incoming().for_each(move |(stream, addr)| {
            println!("Входящее соединение: {}", addr);
            let (reader, writer) = stream.split();
            // Then register our address with the stream to send data to us.
            // Создать канал для потока, который в других сокетов будет
            // послать нам сообщения. Затем зарегистрировать наш адрес с потоком отправить
            // данных в США. RX Receive Data (Принимаемые данные) TX Transmit Data
            let (tx, rx) = mpsc::unbounded::<String>();
            //let (tx_s, rx_s) = mpsc::unbounded::<String>();
            connections.borrow_mut().insert(addr, tx);
            connections_info.borrow_mut().insert(addr, Connect {
                is_new: true,
            });
            //            connections.borrow_mut().insert(
            //                addr,
            //                Connect {
            //                    //stream: TcpStreamCloneable { inner: Rc::new(RefCell::new(stream)) },
            //                    tunnel_data: (tx, rx),
            //                    tunnel_sync: (tx_s, rx_s),
            //                    is_new: true
            //                }
            //            );

            // Определяем, что мы делаем для активного/текущего ввода/вывода.
            // То есть, читаем кучу строк из сокета и обрабатываем их
            // и в то же время мы также записываем любые строки из других сокетов.
            let connections_inner = connections.clone();
            let reader = BufReader::new(reader);

            // Model читает частями из сокета путем mapping-га в бесконечном цикле
            // для каждой строки из сокета. Этот "loop" завершается с ошибкой
            // после того как мы получили EOF на сокете.
            let iter = stream::iter(iter::repeat(()).map(Ok::<(), Error>));
            let socket_reader = iter.fold(reader, move |reader, _| {
                // Прочитать строки из сокета, в противном случае, мы в EOF
                let line = io::read_until(reader, b'\n', Vec::new());
                let line = line.and_then(|(reader, vec)| {
                    if vec.is_empty() {
                        Err(Error::new(ErrorKind::BrokenPipe, "broken pipe"))
                    } else {
                        Ok((reader, vec))
                    }
                });

                // Преобразовать байты в строку, а затем отправить
                // строку всем остальным подключенным клиентам.
                let line = line.map(|(reader, vec)| {
                    (reader, String::from_utf8(vec))
                });
                let connections = connections_inner.clone();
                line.map(move |(reader, message)| {
                    println!("{}: {:?}", addr, message);
                    let mut conns = connections.borrow_mut();
                    if let Ok(msg) = message {
                        // Для каждого открытого соединения, кроме отправителя, отправить
                        // строку через канал.
                        let iter = conns.iter_mut()
                            .filter(|&(&k, _)| k != addr)
                            .map(|(_, v)| v);
                        for tx in iter {
                            tx.send(format!("{}: {}", addr, msg)).unwrap();
                        }
                        // берем записыватель и пишем.
                    } else {
                        let tx = conns.get_mut(&addr).unwrap();
                        tx.send("You didn't send valid UTF-8.".to_string()).unwrap();
                    }
                    reader
                })
            });

            // Всякий раз, когда мы получаем строку на приемнике, мы пишем его
            //`WriteHalf<TcpStream>`.
            let socket_writer = rx.fold(writer, |writer, msg| {
                let amt = io::write_all(writer, msg.into_bytes());
                //let amt = io::write_all(writer, b"\n"); //"\n".as_bytes()
                let amt = amt.map(|(writer, _)| writer);
                amt.map_err(|_| ())
            });

            // Теперь, когда мы получили futures, представляющие каждую половину гнезда, мы
            // используем `select` комбинатор ждать либо половину нужно сделать, чтобы
            // рушить другие. Тогда мы порождать результат.
            let connections = connections.clone();
            let socket_reader = socket_reader.map_err(|_| ());
            let connection = socket_reader.map(|_| ()).select(socket_writer.map(|_| ()));
            handle.spawn(connection.then(move |_| {
                // Для каждого открытого соединения, кроме отправителя, отправить
                // строку через канал.
                let mut conns = connections.borrow_mut();
                {
                    let iter = conns.iter_mut()
                        .filter(|&(&k, _)| k != addr)
                        .map(|(_, v)| v);
                    for tx in iter {
                        tx.send(format!("Клиент {} отвалился.", addr)).unwrap();
                    }
                }
                // ------------------------
                conns.remove(&addr);
                println!("Соединение {} закрыто.", addr);
                Ok(())
            }));


            Ok(())
        });

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