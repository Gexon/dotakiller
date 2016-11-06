use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::rc::Rc;
use std::str;

use byteorder::{ByteOrder, BigEndian};

use mio::*;
use mio::tcp::*;

use ::server::commands as comm;

/// Обертка вокруг неблокирующих сокетов.
/// Это Connection не соединяется с Сервером.
/// Это Connection представляет клиентские подключения,
/// принимаемых Серверными подключениями.
pub struct Connection {
    // handle подключенного сокета
    sock: TcpStream,

    // токен для регистрации в опроснике событий
    pub token: Token,

    // токен авторизации. добывается из базы данных по имени пользователя.
    auth_token: i64,

    // имя пользователя
    name: Vec<u8>,

    // приоритет
    interest: Ready,

    // очередь отправляемых сообщений
    send_queue: Vec<Rc<Vec<u8>>>,

    // флаг перерегистрации
    is_idle: bool,

    // флаг сброса соединения
    is_reset: bool,

    // флаг нового подключения
    is_newbe: bool,

    // флаг чтения получения `WouldBlock`
    // и хранит количество байт что мы должны читать.
    // track whether a read received `WouldBlock` and store the number of
    // byte we are supposed to read
    read_continuation: Option<u64>,

    // флаг записи получил `WouldBlock`
    write_continuation: bool,
}

impl Connection {
    pub fn new(sock: TcpStream, token: Token) -> Connection {
        Connection {
            sock: sock,
            token: token,
            auth_token: 0,
            name: Vec::new(),
            interest: Ready::hup(),
            send_queue: Vec::new(),
            is_idle: true,
            is_reset: false,
            is_newbe: true,
            read_continuation: None,
            write_continuation: false,
        }
    }

    /// Происходит чтение из сокета соединения.
    /// Считываем события из опрашивателя "poller".
    ///
    /// The Handler must continue calling until None is returned.
    /// Обработчик должен продолжать опрашивать, пока ни один не вернулся.
    ///
    /// The recieve buffer is sent back to `Server` so the message can be broadcast to all
    /// listening connections.
    /// В буфере возвращается "сервер", так что сообщение может быть передано на все прослушивания соединений.
    pub fn readable(&mut self) -> io::Result<Option<Vec<u8>>> {
        let msg_len = match try!(self.read_message_length()) {
            None => { return Ok(None); },
            Some(n) => n,
        };

        if msg_len == 0 {
            debug!("в сообщении 0 байт; token={:?}", self.token);
            return Ok(None);
        }

        let msg_len = msg_len as usize;

        debug!("Ожидаемая длина сообщения {}", msg_len);
        let mut recv_buf: Vec<u8> = Vec::with_capacity(msg_len);
        unsafe { recv_buf.set_len(msg_len); }

        // UFCS: resolve "multiple applicable items in scope [E0034]" error
        let sock_ref = <TcpStream as Read>::by_ref(&mut self.sock);

        match sock_ref.take(msg_len as u64).read(&mut recv_buf) {
            Ok(n) => {
                debug!("CONN : считано {} байт", n);
                let mut recv2_buf: Vec<u8> = Vec::with_capacity(msg_len);
                let s = str::from_utf8(&recv_buf[..]).unwrap();
                let data = s.trim();
                let data: Vec<&str> = data.splitn(2, ' ').collect();
                match data[0] {
                    "pos" => {
                        let (smsg, return_token, return_name, return_reset) = comm::pos(
                            data[1], &self.auth_token, self.name.as_slice(), &self.is_reset);
                        println!("{}", smsg);
                        // возвращаемые значения
                        self.auth_token = return_token;
                        self.name = return_name;
                        self.is_reset = return_reset;
                        // вектор с сообщением для рассылки.
                        let smsg_len = smsg.len();
                        recv2_buf = Vec::with_capacity(smsg_len);
                        unsafe { recv2_buf.set_len(smsg_len); }
                        recv2_buf = smsg.into_bytes();
                    },
                    "ping" => {
                        let smsg: String = s.to_string();
                        let smsg_len = smsg.len();
                        recv2_buf = Vec::with_capacity(smsg_len);
                        unsafe { recv2_buf.set_len(smsg_len); }
                        recv2_buf = smsg.into_bytes();
                    },
                    _ => (),
                }

                if n < msg_len as usize {
                    return Err(Error::new(ErrorKind::InvalidData, "Не осилил достаточно байт"));
                }

                self.read_continuation = None;
                //println!("Всем> {}", recv_buf);
                Ok(Some(recv2_buf.to_vec()))
            }
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock {
                    debug!("CONN : прочитал WouldBlock");

                    // Мы вынуждены еще раз попробовать, но мы уже читаем два байта из проволоки,
                    // что определяет Продолжительность
                    // Нам нужно сохранять длину сообщения,
                    // так что мы можем продолжить в следующий раз вызвав функцию readable.
                    self.read_continuation = Some(msg_len as u64);
                    Ok(None)
                } else {
                    error!("Неудалось считать буфер для сокета {:?}, error: {}", self.token, e);
                    Err(e)
                }
            }
        }
    }

    /// Принимаем длину сообщения из сокета соединения.
    fn read_message_length(&mut self) -> io::Result<Option<u64>> {
        if let Some(n) = self.read_continuation {
            return Ok(Some(n));
        }

        let mut buf = [0u8; 8];

        let bytes = match self.sock.read(&mut buf) {
            Ok(n) => {
                //let s = str::from_utf8(&buf[..]).unwrap();
                //println!("Содержимое длины сообщения:{}, количество считанных байт:{}", s, n);
                n
            },
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock {
                    return Ok(None);
                } else {
                    return Err(e);
                }
            }
        };

        if bytes < 8 {
            warn!("Ошибка длины сообщения {} bytes", bytes);
            println!("Ошибка длины сообщения {} bytes", bytes);
            return Err(Error::new(ErrorKind::InvalidData, "Недопустимая длина сообщения"));
        }

        //println!("Содержимое сообщения о длине {:?}", buf.as_ref());
        let msg_len = BigEndian::read_u64(buf.as_ref());


        if msg_len > 1048576 {
            warn!("Ошибка длины сообщения {} bytes", bytes);
            println!("Ошибка длины сообщения {} bytes", bytes);
            println!("Длина входящего сообщения {}", msg_len);
            return Err(Error::new(ErrorKind::InvalidData, "Недопустимая длина сообщения"));
        }

        Ok(Some(msg_len))
    }

    /// Пишем в сокет.
    /// Считываем события для записи из нашего опросника "poller".
    ///
    /// Отправить сообщения из очереди отправки к клиенту.
    /// Если очередь пуста, удалить приоритет в записи событий.
    /// TODO: Выяснить, если посылка более одного сообщения является оптимальным.
    /// Может быть мы должны попытаться спустить... до возвращения сообщения ядром?
    /// Maybe we should be trying to flush until the kernel sends back EAGAIN?
    pub fn writable(&mut self) -> io::Result<()> {
        try!(self.send_queue.pop()
            .ok_or(Error::new(ErrorKind::Other, "Не могу вынуть из очереди отправки сообщений"))
            .and_then(|buf| {
                match self.write_message_length(&buf) {
                    Ok(None) => {
                        // сообщение помещается обратно в очередь, так что мы можем снова попробовать
                        self.send_queue.push(buf);
                        return Ok(());
                    },
                    Ok(Some(())) => {
                        let s = str::from_utf8(&buf[..]).unwrap();
                        let len = s.len();
                        if len > 0 {
                            if s != "ping" {
                                println!("send>{}", s);
                            }
                        };
                        ()
                    },
                    Err(e) => {
                        error!("Сбой отправки буфера {:?}, ошибка: {}", self.token, e);
                        return Err(e);
                    }
                }

                match self.sock.write(&*buf) {
                    Ok(n) => {
                        debug!("CONN : записано {} bytes", n);
                        self.write_continuation = false;
                        Ok(())
                    },
                    Err(e) => {
                        if e.kind() == ErrorKind::WouldBlock {
                            debug!("client flushing buf; WouldBlock");

                            // сообщение помещается обратно в очередь, так что мы можем снова попробовать
                            self.send_queue.push(buf);
                            self.write_continuation = true;
                            Ok(())
                        } else {
                            error!("Сбой отправки буфера для {:?}, ошибка: {}", self.token, e);
                            Err(e)
                        }
                    }
                }
            })
        );

        if self.send_queue.is_empty() {
            self.interest.remove(Ready::writable());
        }

        Ok(())
    }


    /// Пишем длину сообщения в сокет.
    fn write_message_length(&mut self, buf: &Rc<Vec<u8>>) -> io::Result<Option<()>> {
        if self.write_continuation {
            return Ok(Some(()));
        }

        let len = buf.len();
        let mut send_buf = [0u8; 8];
        BigEndian::write_u64(&mut send_buf, len as u64);

        match self.sock.write(&send_buf) {
            Ok(n) => {
                debug!("Отправлена длина сообщения {} bytes", n);
                Ok(Some(()))
            }
            Err(e) => {
                if e.kind() == ErrorKind::WouldBlock {
                    debug!("сброс буфера клиента; WouldBlock");

                    Ok(None)
                } else {
                    error!("Не удалось отправить буфер для {:?}, ошибка: {}", self.token, e);
                    Err(e)
                }
            }
        }
    }

    /// Очереди исходящих Сообщений клиенту.
    ///
    /// This will cause the connection to register interests in write events with the poller.
    /// Это установит приоритет в событиях для записи.
    /// The connection can still safely have an interest in read events.
    /// Подключение по-прежнему безопасно может иметь интерес чтению событий.
    /// Чтение и запись буфера работают независимо друг от друга.
    pub fn send_message(&mut self, message: Rc<Vec<u8>>) -> io::Result<()> {
        trace!("connection send_message; token={:?}", self.token);

        self.send_queue.push(message);

        if !self.interest.is_writable() {
            self.interest.insert(Ready::writable());
        }

        Ok(())
    }


    /// Register interest in read events with poll.
    /// Перерегистрация приоритета на чтение событий из опросника.
    ///
    /// This will let our connection accept reads starting next poller tick.
    /// Наше соединение принимает события чтения, начиная со следующего тика обработчика событий.
    pub fn register(&mut self, poll: &mut Poll) -> io::Result<()> {
        trace!("connection register; token={:?}", self.token);

        self.interest.insert(Ready::readable());

        poll.register(
            &self.sock,
            self.token,
            self.interest,
            PollOpt::edge() | PollOpt::oneshot()
        ).and_then(|(), | {
            self.is_idle = false;
            Ok(())
        }).or_else(|e| {
            error!("Сбой перерегистрации {:?}, {:?}", self.token, e);
            Err(e)
        })
    }

    /// Re-register interest in read events with poll.
    /// Перерегистрация интересов для нашего опросника.
    pub fn reregister(&mut self, poll: &mut Poll) -> io::Result<()> {
        trace!("connection reregister; token={:?}", self.token);

        poll.reregister(
            &self.sock,
            self.token,
            self.interest,
            PollOpt::edge() | PollOpt::oneshot()
        ).and_then(|(), | {
            self.is_idle = false;
            Ok(())
        }).or_else(|e| {
            error!("Ошибка перерегистрации {:?}, {:?}", self.token, e);
            Err(e)
        })
    }

    /// метим соединение для сброса
    pub fn mark_reset(&mut self) {
        trace!("connection mark_reset; token={:?}", self.token);

        self.is_reset = true;
    }

    #[inline]
    pub fn is_reset(&self) -> bool {
        self.is_reset
    }

    #[inline]
    pub fn is_newbe(&self) -> bool {
        self.is_newbe
    }

    pub fn mark_old(&mut self) {
        self.is_newbe = false;
    }


    #[inline]
    pub fn is_idle(&self) -> bool {
        self.is_idle
    }

    pub fn mark_idle(&mut self) {
        trace!("подключение помечено на перерегистрацию(idle); token={:?}", self.token);

        self.is_idle = true;
    }
}