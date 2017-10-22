extern crate mysql;


/// получаем имя из БД, по токену.
pub fn get_name(auth_token: &i64) -> String {
    let mut ret_name = String::from("");
    let pool = mysql::Pool::new("mysql://root:dk@localhost:3306").unwrap();
    let mut stmt0 = pool.prepare("  SELECT name \
                                    FROM dotakiller.accounts \
                                    WHERE token=?", ).unwrap();
    for row in stmt0.execute((auth_token, )).unwrap() {
        ret_name = mysql::from_row::<String>(row.unwrap());
    }
    ret_name
}