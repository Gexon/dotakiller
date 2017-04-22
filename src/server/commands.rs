// обработка токена и что-то еще

use time;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::{RefCell, RefMut};
use std::iter;
use std::io::{Error, ErrorKind, Write};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::str::{self, FromStr};
use std::num::ParseIntError;

use futures::{self, future, Future, Sink, Poll};
use futures::stream::{self, Stream};
use futures::sync::oneshot::Sender;
use futures::sync::mpsc;
use tokio_core::reactor::Core;
use tokio_core::net::{TcpStream, TcpListener};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Decoder, Encoder};

use bytes::{BytesMut, Buf, BufMut, IntoBuf, LittleEndian};

use ::utility::dbqury as db;
use ::server::proto::Connect;
use ::server::proto::Message;

/// Обработка входящих сообщений
pub fn message_pos(message: &Message,
                   connect: &mut Connect,
                   reset: bool)
                   -> Result<Message, String> {
    if let Message::Pos { ref user, x, y } = *message {
        let name = try!(if connect.name.is_empty() {
            connect.token = user.parse().map_err(|e: ParseIntError| e.to_string())?;
            let name = db::get_name(&connect.token);
            if name.is_empty() {
                Err("Не удалось извлечь имя из БД"
                    .to_string())
            } else {
                Ok(name)
            }
        } else {
            Ok(connect.name.clone())
        });

        if check_token(&connect.token) {
            Ok(Message::Pos {
                user: name.clone(),
                x,
                y,
            })
        } else {
            Err("Токен авторизации просрочен".into())
        }
    } else {
        Err("undefined".into())
    }
}

// принимаем позицию клиента и рассылаем всем остальным.
pub fn pos(args: &str, auth_token: &i64, name: &str, reset_conn: bool) -> (String, i64, String, bool) {
    // todo хранить токен в
    // инициализация возвращаемых значений.
    let mut return_msg: String = String::from("");
    let mut return_token: i64 = *auth_token;
    //let return_name: Vec<u8>;
    let mut return_reset: bool = reset_conn;

    // принимаемый аргумент name, передаем строке.
    //let s = str::from_utf8(&name[..]).unwrap();
    //let mut str_name: String = s.to_string();
    let str_name = name;
    let mut string_name: String = str_name.to_string();

    // для отладки.
    //println!("recv>{}", args);

    // вынимаем из пришедшей строки токен. vec_msg[0] - токен авторизации, vec_msg[1] - позиция игрока (x y)
    let args = args.trim();
    let vec_msg: Vec<&str> = args.splitn(2, ' ').collect();
    //    for x in &vec_msg {
    //        println!("vec_msg: {}", x);
    //    }
    //vec_msg.iter().map(|x| println!("{}", x));

    // если соединению не присвоено имя, то вынуть из БД имя по токену.
    if string_name.is_empty() {
        // берем из БД имя по токену.
        let token_64: i64 = i64::from_str(vec_msg[0]).unwrap_or(0);
        return_token = token_64;
        let name_from_db = db::get_name(&token_64);
        string_name = name_from_db;
        if string_name.is_empty() {
            return_reset = true;
            return_msg = "Не удалось извлечь имя из БД.".to_string();
        }
        println!("Имя из БД {}", string_name);
    }

    // если проверка имени прошла успешно, то
    if !return_reset {
        // проверяем актуальность токена
        if check_token(&return_token) {
            let head_msg: &str = "pos";
            return_msg = format!("{} {} {}", head_msg, string_name, vec_msg[1]);
        } else {
            // токен не актуален, порвать соединение.
            return_reset = true;
            return_msg = "Токен авторизации просрочен.".to_string();
        }
    }

    //let smsg_len = str_name.len();
    //let mut recv_buf: Vec<u8> = Vec::with_capacity(smsg_len);
    //unsafe { recv_buf.set_len(smsg_len); }
    //recv_buf = str_name.into_bytes();
    //return_name = recv_buf;

    (return_msg, return_token, string_name, return_reset)
}

// проверяем актуальность токена
pub fn check_token(auth_token: &i64) -> bool {
    // расшифровываем
    // получаем текущую дату и сравниваем не истек ли токен.
    let current_time = time::get_time();
    //println!("current_time {}", current_time.sec);
    //println!("auth_token {}", auth_token);
    if auth_token > &current_time.sec {
        return true
    }

    false
}