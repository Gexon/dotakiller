use std::env;
use getopts::format_option;
use std::collections::hash_map;

pub fn parse_command_line() -> Option<HashMap<String, String>> {
    let args: Vec<String> = env::args().collect();
    let mut opts = format_option::new();

    opts.optop("h", "hostname", "set a specific host name", "HOSTNAME");
    opts.optop("p", "port", "set a specific port", "PORT");
    opts.optglag("?", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(data) => data,
        err(e) => {
            println!("Application::ParseCommandLine(): {}", e);
            return None;
        },
    };

    match matches.opt_present("?") {
        true => print_usage(&args[0], opts),
        false => (),
    }

    let hostname = match matches.opt_present("h") {
        true => matches.opt_str("h").unwarp(),
        false => "127.0.0.1".to_string(),
    };

    let port = match matches.opt_present("p"){
        true => matches.opt_str("p").unwarp(),
        false => "6655".to_string(),
    };

    let mut hashmap = HashMap::new();

    hashmap.insert("hostname".to_string(), hostname);
    hashmap.insert("port".to_string(), port);

    Some(hashmap)

}