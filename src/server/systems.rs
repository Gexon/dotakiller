// тут будем контакт контачить
use tinyecs::*;

use std::io::{self, ErrorKind};
use std::rc::Rc;
use std::net::SocketAddr;

use mio::*;
use mio::tcp::*;
use slab;
use std::time::Duration;

use ::server::connection::Connection;
use ::ground::components::*;
use ::flora::components::FloraState;

type Slab<T> = slab::Slab<T, Token>;

pub struct ServerClass;

impl Component for ServerClass {}

// новый сервак
pub struct ServerSystem {
    server_data: Server,
    poll: Poll,
}

impl ServerSystem {
    pub fn new() -> ServerSystem {
        let hname: &str = "192.168.0.131";
        //let hname: &str = "194.87.237.144";
        let pname: &str = "6655";

        let address = format!("{}:{}", hname, pname);
        let addr = address.parse::<SocketAddr>().expect("Ошибка получения строки host:port");
        let sock = TcpListener::bind(&addr).expect("Ошибка биндинга адреса");

        // Запускаем обработку событий, Poll - опросы событий хранятся внутри сервера.
        //server.run(&mut poll).expect("Ошибка запуска сервера.");

        // Создам объект опроса который будет использоваться сервером для получения событий
        let mut poll = Poll::new().expect("Ошибка создания опросника 'Poll'");

        let mut server = Server {
            sock: sock,

            // Даем нашему серверу токен с номером большим чем может поместиться в нашей плите 'Slab'.
            // Политы 'Slab' используються только для внутреннего смещения.
            token: Token(10_000_000),

            // SERVER is Token(1), после запуска такого сервака
            // мы сможет подключить не больше 128 клиентов.
            conns: Slab::with_capacity(128),

            // Список событий что сервер должен обработать.
            events: Events::with_capacity(1024)
        };

        server.register(&mut poll).expect("I WANT HANDLE ERROR");

        info!("Основной-сервер запущен.");
        println!("Основной-сервер запущен.");

        ServerSystem {
            server_data: server,
            poll: poll,
        }
    }
}

// работа с сетью. передача данных клиентам.
impl System for ServerSystem {
    fn aspect(&self) -> Aspect {
        aspect_all![Graphic]
    }

    fn process_no_entities(&mut self) {
        println!("instaced buffer render system must work, but no entities!");
    }
    fn process_no_data(&mut self) {
        println!("instaced buffer render system must work, but no data!");
    }

    // вызывается 1 раз при update, но для каждой сущности свой process_one
    //fn  process_one(&mut self, _entity: &mut Entity) {

    // вызывается при update, 1 раз для всех сущностей.
    fn process_all(&mut self, entities: &mut Vec<&mut Entity>, _world: &mut WorldHandle, _data: &mut DataList) {
        trace!("PROCESS_ALL start.");
        let cnt = self.poll.poll(&mut self.server_data.events, Some(Duration::from_millis(1000))).expect("do it another day");
        //let cnt = self.poll.poll(&mut self.server_data.events, None).unwrap();
        let mut i = 0;
        //trace!("обработка событий... cnt={}; len={}", cnt, self.server_data.events.len());
        // Перебираем уведомления. Каждое из этих событий дает token для регистрации
        // (который обычно представляет собой, handle события),
        // а также информацию о том, какие события происходили (чтение, запись, сигнал, и т. д.)
        while i < cnt {
            trace!("PROCESS_ALL while.");
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

        // перебираем все сущности
        for entity in entities {
            let mut graphic = entity.get_component::<Graphic>();
            let class = entity.get_component::<Name>();
            let position = entity.get_component::<Position>();
            let state =  entity.get_component::<FloraState>();
            // репликация.
            if graphic.need_replication {
                // рассылаем всем клиентам "updherb idHerb classHerb stateHerb x y"
                let s = format!("updherb 1 {} {} {} {}", class.name, state.state, position.x, position.y);
                let smsg: String = s.to_string();
                let smsg_len = smsg.len();
                let mut recv_buf: Vec<u8> = Vec::with_capacity(smsg_len);
                unsafe { recv_buf.set_len(smsg_len); }
                recv_buf = smsg.into_bytes();
                self.server_data.replication(recv_buf.to_vec());
                graphic.need_replication = false;
                trace!("REPLICATION replication");
            }
            // если есть новенькие, собираем все сущности для primary_replication
            if exist_new_conn {
                let s = format!("updherb 1 {} {} {} {}", class.name, state.state, position.x, position.y);
                let smsg: String = s.to_string();
                let smsg_len = smsg.len();
                let mut recv_buf: Vec<u8> = Vec::with_capacity(smsg_len);
                unsafe { recv_buf.set_len(smsg_len); }
                recv_buf = smsg.into_bytes();
                recv_obj.push(recv_buf.to_vec());
            }
        }

        // первичная рассылка. для вновь подключенных клиентов.
        if exist_new_conn {
            trace!("REPLICATION primary_replication");
            self.server_data.primary_replication(&mut recv_obj);
        }
    }
}


pub struct Server {
    // главное гнездо нашего сервера
    sock: TcpListener,

    // токен нашего сервера. мы отслеживаем его здесь вместо `const SERVER = Token(0)`.
    token: Token,

    // список подключений нашего сервера _accepted_ by
    conns: Slab<Connection>,

    // список событий для обработки
    events: Events,
}

impl Server {
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
    pub fn primary_replication(&mut self, recv_obj: &mut Vec<Vec<u8>>) {
        while !recv_obj.is_empty() {
            if let Some(message) = recv_obj.pop() {
                let rc_message = Rc::new(message);
                for c in self.conns.iter_mut() {
                    if c.is_newbe() {
                        c.send_message(rc_message.clone())
                            .unwrap_or_else(|e| {
                                error!("Сбой записи сообщения в очередь {:?}: {:?}", c.token, e);
                                c.mark_reset();
                            });
                        c.mark_old();
                    }
                }
            }
        }
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
    }

    /// обработчик события
    fn ready(&mut self, poll: &mut Poll, token: Token, event: Ready) {
        //debug!("токен {:?} событие = {:?}", token, event);
        //println!("токен {:?} событие = {:?}", token, event);

        if event.is_error() {
            warn!("Ошибка события токена{:?}", token);
            //println!("Ошибка события токена{:?}", token);
            self.find_connection_by_token(token).mark_reset(); // пометить на сброс соединения
            return;
        }

        if event.is_hup() {
            //trace!("Hup event for {:?}", token);
            //println!("Hup event for {:?}", token);
            self.find_connection_by_token(token).mark_reset();
            return;
        }

        // Мы не обнаружили ошибок записи событий для токена нашего сервера.
        // Запись события для прочих токенов, должны передаваться этому подключению.
        if event.is_writable() {
            //trace!("Записываем событие для токена {:?}", token);
            //println!("Записываем событие для токена {:?}", token);
            assert!(self.token != token, "Получение записанного события для Сервера");

            let conn = self.find_connection_by_token(token);

            if conn.is_reset() {
                info!("{:?} соединение сброшено", token);
                return;
            }

            conn.writable()
                .unwrap_or_else(|e| {
                    warn!("Ошибка записи события для токена {:?}, {:?}", token, e);
                    conn.mark_reset();
                });
        }

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
                Ok(_) => {},
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