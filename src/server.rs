use std::io::{self, ErrorKind};
use std::rc::Rc;

use mio::*;
use mio::tcp::*;
use slab;
use tinyecs::*;

use connection::Connection;

type Slab<T> = slab::Slab<T, Token>;

pub struct Server {
    // главное гнездо нашего сервера
    sock: TcpListener,

    // токен нашего сервера. мы отслеживаем его здесь вместо `const SERVER = Token(0)`.
    token: Token,

    // список подключений нашего сервера _accepted_ by
    conns: Slab<Connection>,

    // список событий для обработки
    events: Events,

    // ECS
    dk_world: World,
}

impl Server {
    pub fn new(sock: TcpListener, in_dk_world: World) -> Server {
        Server {
            sock: sock,

            // Даем нашему серверу токен с номером большим чем может поместиться в нашей плите 'Slab'.
            // Политы 'Slab' используються только для внутреннего смещения.
            token: Token(10_000_000),

            // SERVER is Token(1), после запуска такого сервака
            // мы сможет подключить не больше 128 клиентов.
            conns: Slab::with_capacity(128),

            // Список событий что сервер должен обработать.
            events: Events::with_capacity(1024),

            // ECS
            dk_world: in_dk_world,
        }
    }

    /// Старт сервака собственно.
    /// тут же бесконечный цикл обработки событйи.
    pub fn run(&mut self, poll: &mut Poll) -> io::Result<()> {
        try!(self.register(poll));

        info!("Запуск сервера, запуск цикла...");
        println!("Основной-сервер запущен.");
        loop {
            //println!("run loop");
            let cnt = try!(poll.poll(&mut self.events, None));

            let mut i = 0;

            trace!("обработка событий... cnt={}; len={}", cnt, self.events.len());
            //println!("обработка событий... cnt={}; len={}", cnt, self.events.len());

            // Перебираем уведомления.
            // Каждое из этих событий дает token для регистрации
            // (который обычно представляет собой, handle события),
            // а также информацию о том, какие события происходили (чтение, запись, сигнал, и т. д.)
            while i < cnt {
                //println!("run while");
                // TODO this would be nice if it would turn a Result type. trying to convert this
                // into a io::Result runs into a problem because .ok_or() expects std::Result and
                // not io::Result
                let event = self.events.get(i).expect("Ошибка получения события");

                trace!("event={:?}; idx={:?}", event, i);
                //println!("event={:?}; idx={:?}", event, i);
                self.ready(poll, event.token(), event.kind());

                i += 1;
            }

            self.tick(poll);
            self.dk_world.update();
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
        trace!("register tick");
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
        debug!("токен {:?} событие = {:?}", token, event);
        //println!("токен {:?} событие = {:?}", token, event);

        if event.is_error() {
            warn!("Ошибка события токена{:?}", token);
            //println!("Ошибка события токена{:?}", token);
            self.find_connection_by_token(token).mark_reset(); // пометить на сброс соединения
            return;
        }

        if event.is_hup() {
            trace!("Hup event for {:?}", token);
            //println!("Hup event for {:?}", token);
            self.find_connection_by_token(token).mark_reset();
            return;
        }

        // Мы не обнаружили ошибок записи событий для токена нашего сервера.
        // Запись события для прочих токенов, должны передаваться этому подключению.
        if event.is_writable() {
            trace!("Записываем событие для токена {:?}", token);
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
            trace!("Читаем событие для токена {:?}", token);
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
        debug!("сервер принимает новый сокет");

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
                    debug!("регистрация {:?} в опроснике", entry.index());
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
        debug!("сервер conn доступен; token={:?}", token);

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