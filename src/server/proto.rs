// тут будут всякие ништяки для сервера

use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use std::str::{self, FromStr};

use futures::sync::oneshot::Sender;
use futures::sync::mpsc;
use tokio_io::codec::{Decoder, Encoder};

use bytes::{BytesMut, Buf, BufMut, IntoBuf, LittleEndian};

pub type Connections = HashMap<SocketAddr, mpsc::UnboundedSender<Result<Message, MessageError>>>;

/// Храним состояние соединения и т.п.
pub struct Connect {
    //stream: TcpStreamCloneable,
    pub is_new: bool,
    pub token: i64,
    pub name: String,
}

// незнаю для чего)
pub struct MessageCodec;

// собственно сообщение
#[derive(Clone, Debug)]
pub enum Message {
    Pos { user: String, x: f32, y: f32 },
    DelPos { user: String },
    UpdHerb {
        id: i32,
        class: String,
        state: i32,
        x: f32,
        y: f32,
    },
    UpdMonster {
        id: i32,
        class: String,
        state: i32,
        x: f32,
        y: f32,
        emo: i32,
    },
    Ping,
    Raw(String),
    Error(String),
}

// запись буфер видимо
impl Message {
    fn write_to_buf(&self, buf: &mut BytesMut) -> Result<(), Error> {
        match *self {
            Message::Pos { ref user, x, y } => {
                buf.extend(format!("pos {} {} {}", user, x, y).as_bytes());

                Ok(())
            }
            Message::DelPos { ref user } => {
                buf.extend(b"delpos ");
                buf.extend(user.as_bytes());

                Ok(())
            }
            Message::UpdHerb {
                id,
                ref class,
                state,
                x,
                y,
            } => {
                buf.extend(format!("updherb {} {} {} {} {}", id, class, state, x, y).as_bytes());

                Ok(())
            }
            Message::UpdMonster {
                id,
                ref class,
                state,
                x,
                y,
                emo,
            } => {
                buf.extend(format!("updmonstr {} {} {} {} {} {}", id, class, state, x, y, emo).as_bytes());

                Ok(())
            }
            Message::Ping => {
                buf.extend(b"ping");
                Ok(())
            }
            Message::Raw(ref message) => {
                buf.extend(message.as_bytes());
                Ok(())
            }
            Message::Error(ref error) => {
                buf.extend(error.as_bytes());
                Ok(())
            }
        }
    }

}

// сильная магия от bredov XD
macro_rules! parse_message {
    ($iter:ident $(, $id:ident)*) => (
        $(
            let $id = $iter
                .next()
                .ok_or(concat!("Message::from_str: missing `", stringify!($id), "'."))?;
            let $id = $id
                .parse()
                .map_err(|_|
                    concat!("Message::from_str: invalid `", stringify!($id), "'."))?;
        )*
    );
}

// тут магия со строками
impl FromStr for Message {
    type Err = (String);

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //let mut words_iter = s.split(' ');
        let mut words_iter = s.split(' ').filter(|word| !word.is_empty());
        let header = words_iter
            .next()
            .ok_or("Message::from_str: missing header")?;

        let message = match header {
            "pos" => {
                parse_message!(words_iter, user, x, y);

                Message::Pos { user, x, y }
            }

            "delpos" => {
                parse_message!(words_iter, user);

                Message::DelPos { user }
            }

            "updherb" => {
                parse_message!(words_iter, id, class, state, x, y);

                Message::UpdHerb {
                    id,
                    class,
                    state,
                    x,
                    y,
                }
            }

            "updmonstr" => {
                parse_message!(words_iter, id, class, state, x, y, emo);

                Message::UpdMonster {
                    id,
                    class,
                    state,
                    x,
                    y,
                    emo,
                }
            }

            "ping" => Message::Ping,

            raw => Message::Raw(raw.to_string()),
            // Message::Error(format!("unknown message `{}'", unknown)),
        };

        Ok(message)
    }
}

// при приёме сообщения
impl Decoder for MessageCodec {
    type Item = Message;
    type Error = MessageError;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if buf.len() < 8 {
            return Ok(None);
        }

        let size = {
            let size = buf[..8].into_buf().get_u64::<LittleEndian>() as usize;

            if size == 0 || size > 1048576 {
                let error = Error::new(ErrorKind::InvalidData,
                                       format!("Неверный размер данных: {}",
                                               size));
                return Err(MessageError::from(error));
            }

            if buf.len() < 8 + size {
                return Ok(None);
            }

            size
        };

        buf.split_to(8); // Skip size// split_to делит буфер на две части, возвращая первые N байт, смещая при этом курсор

        let raw_data = buf.split_to(size);
        let data = str::from_utf8(&raw_data);

        match data {
            Ok(data) => {
                let message = data.parse();

                match message {
                    Ok(message) => Ok(Some(message)),
                    Err(error) => {
                        Err(MessageError::from(Error::new(ErrorKind::InvalidData,
                                                          error.to_string())))
                    }
                }
            }
            Err(error) => {
                Err(MessageError::from(Error::new(ErrorKind::InvalidData, error.to_string())))
            }
        }
    }
}

//
pub struct MessageError {
    pub inner: Error,
    pub complete: Option<Sender<()>>,
}

//
impl MessageError {
    pub fn _new(error: String) -> MessageError {
        let error = Error::new(ErrorKind::Other, error);
        MessageError::from(error)
    }
}

//
impl From<Error> for MessageError {
    fn from(e: Error) -> MessageError {
        MessageError {
            inner: e,
            complete: None,
        }
    }
}

//
impl ToString for MessageError {
    fn to_string(&self) -> String {
        self.inner.to_string()
    }
}

// пр отправке данных
impl Encoder for MessageCodec {
    type Item = Message;
    type Error = MessageError;

    fn encode(&mut self, message: Self::Item, buf: &mut BytesMut) -> Result<(), Self::Error> {
        let mut msg_buf = BytesMut::with_capacity(20);
        try!(message
            .write_to_buf(&mut msg_buf)
            .map_err(MessageError::from));
        buf.reserve(8 + msg_buf.len());
        buf.put_u64::<LittleEndian>(msg_buf.len() as u64);
        buf.put(msg_buf);
        Ok(())
    }
}