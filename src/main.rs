use std::io;

fn main() {
		input_number();


}

fn input_number() {
	println!("Угадай загаданное мной число");
	println!("Введите число");

	let mut number = String::new();

					io::stdin().read_line(&mut number)
			.ok()
		.expect("ошибка ввода какая-то");

	println!("Ты угадал! {}", number);
}