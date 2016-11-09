// общие копоненты для всех сущностей.

use tinyecs::*;
use time::PreciseTime;

pub struct Name {
    pub name: String
}

impl Component for Name {}

pub struct ClassGround;

impl Component for ClassGround {}

// храним последние id
pub struct WorldLastId {
    pub flora_id: i64,
}

impl Component for WorldLastId {}
// тут будем хранить все объекты на карте.
pub struct WorldMap {
    pub flora: [&[f64];3] [u8; 140],
}

impl Component for WorldMap {
    //    fn foo(a:&[&[f64]],x:&[f64])
    //    {
    //        for i in 0..3 {
    //            for j in 0..4 {
    //                println!("{}",a[i][j]);
    //            }
    //        }
    //    }

    //        let a:[&[f64];3]=[&[1.1,-0.2, 0.1,1.6],
    //            &[0.1,-1.2,-0.2,2.3],
    //            &[0.2,-0.1, 1.1,1.5]];
    //
    //let x:[f64;3]=[0.0;3];
    //        foo(&a,&x);
}

// куда дует ветер. 0 - это типа север(+Х)
pub struct WindDirection {
    pub direction: u8,
    pub start: PreciseTime,
}

impl Component for WindDirection {}

// репликация клиенту изменений.
pub struct Graphic {
    pub need_replication: bool,
}

impl Component for Graphic {
    // представление в графической подсистеме
    // репликация клиенту изменений.
}

// сложная хрень.
pub struct _Physic;

impl Component for _Physic {
    // представление в физической подсистеме
    // обработка входящих событий, обработка внутренних процессов, изменение состояний.
    // биология тут же будет
}

pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {}

pub struct SpawnPoint {
    pub name: &'static str,
    pub x: f32,
    pub y: f32,
}

impl Component for SpawnPoint {}