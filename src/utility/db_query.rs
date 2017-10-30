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


/// получаем из БД граф монстра
// пихаем в вектор куски массива по два числа Vec<(i32, i32)>
pub fn get_monster_graph() -> Vec<(i32, i32)> {
    //
    let context = mysql::Pool::new("mysql://root:dk@localhost:3306").unwrap();
    let execution = context.prep_exec("SELECT edge_1, edge_2\
                                            FROM dotakiller.graph_monster\
                                            ", ()).unwrap();

    let data: Vec<(i32, i32)> = execution.map(|u_result| {
        let mut result = u_result.unwrap();
        let edge1: i32 = result.take::<i32, &str>("edge_1").unwrap();
        let edge2: i32 = result.take::<i32, &str>("edge_2").unwrap();
        (edge1, edge2)
    }
    ).collect();

    data
    /*
    pub fn foo(context: &GlobalContext, plname: String) -> bool {
      let execution = context.db_pool.prep_exec("select * from players where name = ? limit 1", (plname,)).unwrap();

      let data: Vec<Player> = execution.map( |u_result| {
          let mut result = u_result.unwrap();

          let x: String = result.take::<String, &str>("name").unwrap();

          Player { name: x }
        }
      ).collect();

      data.len() > 0
    }
    */
}