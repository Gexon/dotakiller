// тут будем контакт контачить

use tinyecs::*;

use std::io::{self, ErrorKind};
use std::rc::Rc;
use std::net::SocketAddr;
use std::str;

use mio::*;
use mio::tcp::*;
use slab;
use std::time::Duration;

use SERVER_IP;

use ::server::connection::Connection;
use ::server::components::ReplicationServerClass;
use ::ground::components::*;
use ::flora::components::FloraState;
use ::flora::components::HerbId;
use ::flora::components::FloraClass;
use ::monster::components::MonsterId;
use ::monster::components::MonsterClass;
use ::monster::components::MonsterState;

type Slab<T> = slab::Slab<T, Token>;


// сервер репликации
pub struct ReplicationServerSystem {
    server_data: ReplicationServer,
    poll: Poll,
}

impl ReplicationServerSystem {
    pub fn new() -> ReplicationServerSystem {
        let hostname: &str = SERVER_IP;
        let port: &str = "6655";

        let address = format!("{}:{}", hostname, port);
        let addr = address.parse::<SocketAddr>().expect("Ошибка получения строки host:port");
        let sock = TcpListener::bind(&addr).expect("Ошибка биндинга адреса");

        // Запускаем обработку событий, Poll - опросы событий хранятся внутри сервера.
        //server.run(&mut poll).expect("Ошибка запуска сервера.");

        // Создам объект опроса который будет использоваться сервером для получения событий
        let mut poll = Poll::new().expect("Ошибка создания опросника 'Poll'");

        let mut server = ReplicationServer {
            sock: sock,

            // Даем нашему серверу токен с номером большим чем может поместиться в нашей плите 'Slab'.
            // Политы 'Slab' используються только для внутреннего смещения.
            token: Token(10_000_000),

            // SERVER is Token(1), после запуска такого сервака
            // мы сможет подключить не больше 128 клиентов.
            conns: Slab::with_capacity(128),

            // Список событий что сервер должен обработать.
            events: Events::with_capacity(1024),

            // вектор с потраченными игроками
            player_potra4eno: Vec::new(),
            // Флаг потери соединения или отключения клиента.
            flag_potra4eno: false,
        };

        server.register(&mut poll).expect("I WANT HANDLE ERROR");

        info!("Основной-сервер запущен.");
        println!("Основной-сервер запущен.");

        ReplicationServerSystem {
            server_data: server,
            poll: poll,
        }
    }
}


// работа с сетью. передача данных клиентам.
impl System for ReplicationServerSystem {
    fn aspect(&self) -> Aspect {
        aspect_all!(ReplicationServerClass)
    }

    fn data_aspects(&self) -> Vec<Aspect> {
        vec![aspect_all![FloraClass].optional(), aspect_all![MonsterClass].optional()]
    }

    //    fn process_no_entities(&mut self) {
    //        //println!("instaced buffer render system must work, but no entities!");
    //    }
    //    fn process_no_data(&mut self) {
    //        //println!("instaced buffer render system must work, but no data!");
    //    }

    // Вынимаем аспекты макросом, т.к. там безумие в коде.
    impl_process!(self, _entity, | _replication_server_class: ReplicationServerClass | with (_floras, _monsters) => {
        let cnt = self.poll.poll(&mut self.server_data.events, Some(Duration::from_millis(10))).expect("do it another day");
        //let cnt = self.poll.poll(&mut self.server_data.events, None).unwrap();
        let mut i = 0;
        //trace!("обработка событий... cnt={}; len={}", cnt, self.server_data.events.len());
        // Перебираем уведомления. Каждое из этих событий дает token для регистрации
        // (который обычно представляет собой, handle события),
        // а также информацию о том, какие события происходили (чтение, запись, сигнал, и т. д.)
        while i < cnt {
            let event = self.server_data.events.get(i).expect("Ошибка получения события");
            //trace!("event={:?}; idx={:?}", event, i);
            self.server_data.ready(&mut self.poll, event.token(), event.kind());
            i += 1;
        }




        self.server_data.tick(&mut self.poll);

        // вектор векторов, для primary_replication. в нем храним все объекты с карты.
        let mut recv_obj: Vec<Vec<u8>> = Vec::new();
        // определяем наличие свежих подключений, тербующих primary_replication
        let exist_new_conn = self.server_data.exist_new_conn();

        // репликация растений.
        for flora in _floras {
            let id_herb = flora.get_component::<HerbId>();
            let class = flora.get_component::<Name>();
            let position = flora.get_component::<Position>();
            let state = flora.get_component::<FloraState>();

            if flora.has_component::<Replication>() {
                let s = format!("updherb {} {} {} {} {}", id_herb.id, class.name, state.state, position.x, position.y);
                let smsg: String = s.to_string();
                let recv_buf = smsg.into_bytes();
                self.server_data.replication(recv_buf.to_vec());
                flora.remove_component::<Replication>(); // убираем компонент репликации.
                flora.refresh();
            }

            // если есть новенькие клиенты, собираем все сущности для primary_replication
            if exist_new_conn {
                let s = format!("updherb {} {} {} {} {}", id_herb.id, class.name, state.state, position.x, position.y);
                let smsg: String = s.to_string();
                let recv_buf2 = smsg.into_bytes();
                recv_obj.push(recv_buf2.to_vec());
                // отсылаем по 500 штук.
                if recv_obj.len() > 100 {
                    //trace!("REPLICATION primary_replication_100 FLORA, Длина объектов: {}", recv_obj.len());
                    self.server_data.primary_replication(&mut recv_obj, false);
                }
            }
        }

        // репликация монстров.
        for monster in _monsters {
            let id_monstr = monster.get_component::<MonsterId>();
            let class = monster.get_component::<Name>();
            let state = monster.get_component::<MonsterState>();
            let position = monster.get_component::<Position>();

            if monster.has_component::<Replication>() {
                let s = format!("updmonstr {} {} {} {} {}", id_monstr.id, class.name, state.state, position.x, position.y);
                let smsg: String = s.to_string();
                let recv_buf = smsg.into_bytes();
                self.server_data.replication(recv_buf.to_vec()); // репликация
                monster.remove_component::<Replication>(); // убираем компонент репликации.
                monster.refresh();
            }

            // если есть новенькие клиенты, собираем все сущности для primary_replication
            if exist_new_conn {
                let s = format!("updmonstr {} {} {} {} {}", id_monstr.id, class.name, state.state, position.x, position.y);
                let smsg: String = s.to_string();
                let recv_buf2 = smsg.into_bytes();
                recv_obj.push(recv_buf2.to_vec());
                // отсылаем по 500 штук.
                if recv_obj.len() > 100 {
                    //trace!("REPLICATION primary_replication_100 MONSTER");
                    self.server_data.primary_replication(&mut recv_obj, false); // первичная репликация
                }
            }
        }

        if exist_new_conn {
            // Выполняем первичную репликацию, на тот случай если объектов меньше 500.
            self.server_data.primary_replication(&mut recv_obj, true); // первичная репликация
        }
    });
}


pub struct ReplicationServer {
    // главное гнездо нашего сервера
    sock: TcpListener,
    // токен нашего сервера. мы отслеживаем его здесь вместо `const SERVER = Token(0)`.
    token: Token,
    // список подключений нашего сервера _accepted_ by
    conns: Slab<Connection>,
    // список событий для обработки
    events: Events,
    // вектор с именами отключенных клиентов.
    player_potra4eno: Vec<Vec<u8>>,
    // флаг потери клиента, рассылаем всем собщение.
    flag_potra4eno: bool,
}

impl ReplicationServer {
    /// Рассылаем изменения объектов всем подключенным клиентам.
    pub fn replication(&mut self, message: Vec<u8>) {
        let rc_message = Rc::new(message);
        for c in self.conns.iter_mut() {
            c.send_message(rc_message.clone())
                .unwrap_or_else(|e| {
                    error!("Сбой записи сообщения в очередь {:?}: {:?}", c.token, e);
                    c.mark_reset();
                });
        }
    }

    /// оперделяем наличие новых соединений.
    pub fn exist_new_conn(&mut self) -> bool {
        let mut exist_new: bool = false;
        for c in self.conns.iter_mut() {
            if c.is_newbe() {
                exist_new = true;
            }
        }
        exist_new
    }

    /// первичная репликация
    pub fn primary_replication(&mut self, recv_obj: &mut Vec<Vec<u8>>, mark_old: bool) {
        for c in self.conns.iter_mut() {
            if c.is_newbe() {
                while !recv_obj.is_empty() {
                    if let Some(message) = recv_obj.pop() {
                        let rc_message = Rc::new(message);
                        c.send_message(rc_message.clone())
                            .unwrap_or_else(|e| {
                                error!("Сбой записи сообщения в очередь {:?}: {:?}", c.token, e);
                                c.mark_reset();
                            });
                    }
                }
                if mark_old { c.mark_old(); }
            }
        }
    }


    /// Заполняем вектор с потраченными клиентами.
    // с отключенными, для оповещения остальным об этом.
    pub fn filling_potra4eno(&mut self, token: &Token) {
        // разослать всем, что игрок вышел
        let vec_name: Vec<u8>;
        vec_name = self.find_connection_by_token(*token).get_name();
        let s_vec = str::from_utf8(&vec_name[..]).unwrap();
        let s = format!("delpos {}", s_vec);
        //println!("отключение клиента {}", s);
        let s_msg: String = s.to_string();
        let message = s_msg.into_bytes();
        self.player_potra4eno.push(message);
        self.flag_potra4eno = true;
    }

    /// Регистрация серверного опросника событий.
    ///
    /// This keeps the registration details neatly tucked away inside of our implementation.
    /// Это хранит регистрационные данные аккуратно спрятан внутри нашей реализации.
    pub fn register(&mut self, poll: &mut Poll) -> io::Result<()> {
        poll.register(
            &self.sock,
            self.token,
            Ready::readable(),
            PollOpt::edge()
        ).or_else(|e| {
            error!("Ошибка регистрации опросника событий {:?}, {:?}", self.token, e);
            //println!("Ошибка регистрации опросника событий {:?}, {:?}", self.token, e);
            Err(e)
        })
    }

    // конечный такт бесконечного цикла
    fn tick(&mut self, poll: &mut Poll) {
        //trace!("register tick");
        //println!("register tick");

        let mut reset_tokens = Vec::new();

        for c in self.conns.iter_mut() {
            if c.is_reset() {
                //println!("register tick 2");
                reset_tokens.push(c.token);
                //println!("register tick 3");
            } else if c.is_idle() {
                c.reregister(poll)
                    .unwrap_or_else(|e| {
                        warn!("Ошибка регистрации {:?}", e);
                        //println!("Ошибка регистрации {:?}", e);
                        c.mark_reset();
                        reset_tokens.push(c.token);
                    });
            }
        }

        for token in reset_tokens {
            match self.conns.remove(token) {
                Some(_c) => {
                    debug!("сброс соединения; token={:?}", token);
                    //println!("сброс соединения; token={:?}", token);
                }
                None => {
                    warn!("Неудалось удалить соединение для {:?}", token);
                    //println!("Неудалось удалить соединение для {:?}", token);
                }
            }
        }

        if self.flag_potra4eno {
            let recv_obj = &mut self.player_potra4eno;
            for c in self.conns.iter_mut() {
                while !recv_obj.is_empty() {
                    if let Some(message) = recv_obj.pop() {
                        let rc_message = Rc::new(message);
                        c.send_message(rc_message.clone())
                            .unwrap_or_else(|e| {
                                error!("Сбой записи сообщения в очередь {:?}: {:?}", c.token, e);
                                c.mark_reset();
                            });
                    }
                }
            }
            self.flag_potra4eno = false;
        }
    }

    /// обработчик события
    fn ready(&mut self, poll: &mut Poll, token: Token, event: Ready) {
        //debug!("токен {:?} событие = {:?}", token, event);
        //println!("токен {:?} событие = {:?}", token, event);
        //let do_potra4eno: bool = false;


//        if event.is_error() {
//            warn!("Ошибка события токена{:?}", token);
//            //println!("Ошибка события токена{:?}", token);
//            self.filling_potra4eno(&token);
//            self.find_connection_by_token(token).mark_reset(); // пометить на сброс соединения
//            return;
//        }

//        if event.is_hup() {
//            //trace!("Hup event for {:?}", token);
//            //println!("Hup event for {:?}", token);
//            self.filling_potra4eno(&token);
//            self.find_connection_by_token(token).mark_reset();
//            return;
//        }

        let mut do_potra4eno = false;
        // Мы не обнаружили ошибок записи событий для токена нашего сервера.
        // Запись события для прочих токенов, должны передаваться этому подключению.
        if event.is_writable() {
            //trace!("Записываем событие для токена {:?}", token);
            //println!("Записываем событие для токена {:?}", token);
            assert! ( self.token != token, "Получение записанного события для Сервера");

            let conn = self.find_connection_by_token(token);

            if conn.is_reset() {
                info!("{:?} соединение сброшено", token);
                return;
            }

            conn.writable()
                .unwrap_or_else(|e| {
                    warn!("Ошибка записи события для токена {:?}, {:?}", token, e);
                    do_potra4eno = true;
                    conn.mark_reset();
                });
        }

        if do_potra4eno { self.filling_potra4eno(&token); }

        // A read event for our `Server` token means we are establishing a new connection.
        // Событие чтения для токена нашего сервера означает, что мы устанавливаем новое соединение.
        // A read event for any other token should be handed off to that connection.
        // Событие чтения для любого другого токена должны быть передан этому соединению.
        if event.is_readable() {
            //trace!("Читаем событие для токена {:?}", token);
            if self.token == token {
                self.accept(poll);
            } else {
                if self.find_connection_by_token(token).is_reset() {
                    info!("{:?} соединение сброшено", token);
                    return;
                }

                self.readable(token)
                    .unwrap_or_else(|e| {
                        warn!("Ошибка чтения события для токена {:?}: {:?}", token, e);
                        self.filling_potra4eno(&token);
                        self.find_connection_by_token(token).mark_reset();
                    });
            }
        }

        if self.token != token {
            self.find_connection_by_token(token).mark_idle();
        }
    }

    /// Принять новое клиентское соединение.
    ///
    /// The server will keep track of the new connection and forward any events from the poller
    /// to this connection.
    /// Сервер будет отслеживать новое подключение и перенаправлять любые события из опроса
    /// из данного подключения.
    fn accept(&mut self, poll: &mut Poll) {
        debug!("Сервер принимает новый сокет");

        loop {
            // Отчет об ошибке, если нет сокета, иначе двигаемся дальше,
            // поэтому мы не рушить весь сервер.
            let sock = match self.sock.accept() {
                Ok((sock, _)) => sock,
                Err(e) => {
                    if e.kind() == ErrorKind::WouldBlock {
                        debug!("принять столкнулись с блокировкой");
                    } else {
                        error!("Сбой принятия нового сокета, {:?}", e);
                    }
                    return;
                }
            };

            let token = match self.conns.vacant_entry() {
                Some(entry) => {
                    debug!("регистрация {:?} в пуле", entry.index());
                    let c = Connection::new(sock, entry.index());
                    entry.insert(c).index()
                }
                None => {
                    error!("Не удалось вставить соединение в Slab");
                    return;
                }
            };

            match self.find_connection_by_token(token).register(poll) {
                Ok(_) => {}
                Err(e) => {
                    error!("Сбой регистрации {:?} соединения в опроснике событий, {:?}", token, e);
                    self.conns.remove(token);
                }
            }
        }
    }

    /// Перенаправление доступного события в установленное соединение.
    ///
    /// Connections are identified by the token provided to us from the poller.
    /// Соединения определенных токенов, которые мы получили из опросника poller.
    /// Once a read has finished, push the receive buffer into
    /// the all the existing connections so we can broadcast.
    /// После чтения завершится, помещаем буфер приема на все существующие подключения,
    /// чтобы мы могли транслировать.
    fn readable(&mut self, token: Token) -> io::Result<()> {
        //debug!("сервер conn доступен; token={:?}", token);

        while let Some(message) = try!(self.find_connection_by_token(token).readable()) {
            let rc_message = Rc::new(message);
            // Очередь записи для всех подключенных клиентов.
            for c in self.conns.iter_mut() {
                c.send_message(rc_message.clone())
                    .unwrap_or_else(|e| {
                        error!("Сбой записи сообщения в очередь {:?}: {:?}", c.token, e);
                        c.mark_reset();
                    });
            }
        }

        Ok(())
    }

    /// Поиск соединения в Slab с помощью токена.
    fn find_connection_by_token(&mut self, token: Token) -> &mut Connection {
        &mut self.conns[token]
    }
}