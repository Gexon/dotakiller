// регаем/создаем логгер

use std::io;
use std::fs::OpenOptions;

use slog::DrainExt;
use slog;
use slog_stream;
use slog_stdlog;

struct LogFormat;

impl slog_stream::Format for LogFormat {
    fn format(&self,
              io: &mut io::Write,
              rinfo: &slog::Record,
              _logger_values: &slog::OwnedKeyValueList)
              -> io::Result<()> {
        let msg = format!("{} - {}\n", rinfo.level(), rinfo.msg());
        try!(io.write_all(msg.as_bytes()));
        Ok(())
    }
}

pub fn init(){
    // Регистрируем логгер для дебага и статистики.
    let log_path = "dk_server.log";
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_path).unwrap();

    let drain = slog_stream::stream(file, LogFormat).fuse();
    let logger = slog::Logger::root(drain, o!());
    slog_stdlog::set_logger(logger).unwrap();
}