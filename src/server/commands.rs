use time;
use std::str;
use std::str::FromStr;

use ::utility::dbqury as db;

// принимаем позицию клиента и рассылаем всем остальным.
pub fn pos(args: &str, auth_token: &i64, name: &[u8], reset_conn: &bool) -> (String, i64, Vec<u8>, bool) {
    // инициализация возвращаемых значений.
    let mut return_msg: String = String::from("");
    let mut return_token: i64 = *auth_token;
    let return_name: Vec<u8>;
    let mut return_reset: bool = *reset_conn;

    // принимаемый аргумент name, передаем строке.
    let s = str::from_utf8(&name[..]).unwrap();
    let mut str_name: String = s.to_string();

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
    if str_name.is_empty() {
        // берем из БД имя по токену.
        let token_64: i64 = i64::from_str(vec_msg[0]).unwrap_or(0);
        return_token = token_64;
        let name_from_db = db::get_name(&token_64);
        str_name = name_from_db;
        if str_name.is_empty() {
            return_reset = true;
            return_msg = "Не удалось извлечь имя из БД.".to_string();
        }
        println!("Имя из БД {}", str_name);
    }

    // если проверка имени прошла успешно, то
    if !return_reset {
        // проверяем актуальность токена
        if check_token(&return_token) {
            let head_msg: &str = "pos";
            return_msg = format!("{} {} {}", head_msg, str_name, vec_msg[1]);
        } else {
            // токен не актуален, порвать соединение.
            return_reset = true;
            return_msg = "Токен авторизации просрочен.".to_string();
        }
    }

    let smsg_len = str_name.len();
    let mut recv_buf: Vec<u8> = Vec::with_capacity(smsg_len);
    unsafe { recv_buf.set_len(smsg_len); }
    recv_buf = str_name.into_bytes();
    return_name = recv_buf;

    (return_msg, return_token, return_name, return_reset)
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