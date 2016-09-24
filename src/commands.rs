use std::net::TcpStream;

pub fn login(stream: &mut TcpStream, args: &[&str]) -> bool {
    if args.len() != 2 { return false }

    println!("Имя {}, пароль {}", args[0], args[1]);

    // TODO: Use RustORM for connect SQL db and check user exists
    // TODO:

    true
}