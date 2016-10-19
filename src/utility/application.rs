use std::env;
use getopts::Options;
use std::collections::HashMap;

fn print_usage(program_name: &String, opts: Options){
    let brief = format!("Usage: {} [option]", program_name);
    print!("{}", opts.usage(&brief));
}

pub fn parse_command_line() -> Option<HashMap<String, String>> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();

    opts.optopt("h", "hostname", "set a specific host name", "HOSTNAME");
    opts.optopt("p", "port", "set a specific port", "PORT");
    opts.optflag("?", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(data) => data,
        Err(e) => {
            println!("Application::ParseCommandLine(): {}", e);
            return None;
        },
    };

    match matches.opt_present("?") {
        true => print_usage(&args[0], opts),
        false => (),
    }

    let hostname = match matches.opt_present("h") {
        true => matches.opt_str("h").unwrap(),
        false => "192.168.0.3".to_string(),
    };

    let port = match matches.opt_present("p"){
        true => matches.opt_str("p").unwrap(),
        false => "6655".to_string(),
    };

    let mut hashmap = HashMap::new();

    hashmap.insert("hostname".to_string(), hostname);
    hashmap.insert("port".to_string(), port);

    Some(hashmap)

}