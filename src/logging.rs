use log::{self, LevelFilter, Log, Metadata, Record};
use once_cell::sync::OnceCell;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct SimpleLogger {
    file: Option<File>,
}

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let s = format!("[{}] {}\n", record.level(), record.args());
        print!("{}", s);
        if self.file.is_some() {
            self.file.as_ref().unwrap().write(s.as_bytes()).unwrap();
        }
    }
    fn flush(&self) {}
}

impl SimpleLogger {
    pub fn new(file_name: Option<&str>) -> Self {
        let file = if let Some(name) = file_name {
            Some(File::create(name).unwrap())
        } else {
            None
        };
        Self { file }
    }
}

static LOGGER: OnceCell<SimpleLogger> = OnceCell::new();

pub fn init(file_name: Option<&str>) {
    let logger = SimpleLogger::new(file_name);
    LOGGER.set(logger).expect("init log error");
    // let logger = LOGGER.get().unwrap();
    log::set_logger(LOGGER.get().unwrap()).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}
