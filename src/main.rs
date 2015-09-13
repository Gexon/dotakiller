use std::f32::consts;

static MAX_HELTH: i32 = 100;
static GAME_NAME: &'static str = "Убийца доты!";

fn main() {
    dota_mast_die();
    player();
}


// комент, тестовая функция.
fn dota_mast_die(){
    type ManaType = i16;
    let mut num: i32;
    let index = 1;
    num = 1000;
    const PI: f32 = 123.123;
    num = num + MAX_HELTH;
    let mut ibool: i16;
    ibool = num as i16;
    let mana: ManaType = 50;
    ibool = mana;
    println!("Dota mast die! {1} {0} {2}", MAX_HELTH, GAME_NAME, PI);
    println!("Dota mast die! {0} {points}", consts::PI, points=num);
    println!("{}", ibool);
    println!("{:p} {:p}", &ibool, &GAME_NAME);
}

fn player() {
    let player1 = "a1983";
    let player2 = "CheHvost";
    let player3 = "c0deum";
    let player4 = player1.to_string() + player2;
    println!("{} {}", player3, player4);
}


#[test]
fn test_dota_mast_die() {

}