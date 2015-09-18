extern crate rand;

use std::thread;
use std::sync::{Mutex, Arc};
use rand::Rng;

fn main() {
	let table = Arc::new(Table { forks: vec![
	Mutex::new(()),
	Mutex::new(()),
	Mutex::new(()),
	Mutex::new(()),
	Mutex::new(()),
	Mutex::new(()),
	]
	});

	let philosophers = vec![
	Philosopher::new("Markizz", 0, 1),
	Philosopher::new("c0deum", 1, 2),
	Philosopher::new("GoodGod", 2, 3),
	Philosopher::new("hi4C0CK", 3, 4),
	Philosopher::new("0sk0L0k", 4, 5),
	Philosopher::new("a1983", 5, 0),
	];

	let handles: Vec<_> = philosophers.into_iter().map(|p| {
			let table = table.clone();

			thread::spawn(move || {
				p.eat(&table);
		})
	}).collect();

	for h in handles {
				h.join().unwrap();
	}
}

struct Philosopher {
name: String,
left: usize,
right: usize,
}

struct Table {
forks: Vec<Mutex<()>>,
}

impl Philosopher {
fn new(name: &str, left: usize, right: usize) -> Philosopher {
	Philosopher{
	name: name.to_string(),
	left: left,
	right: right,
	}
}

fn eat(& self, table: &Table){
	let _left = table.forks[self.left].lock().unwrap();
	let _right = table.forks[self.right].lock().unwrap();

	println!("{} приступил к питанию", self .name);
	let eat_time = rand::thread_rng().gen_range(500, 2000);
	thread::sleep_ms(eat_time);
	println!("{} кончил", self .name);
}
}