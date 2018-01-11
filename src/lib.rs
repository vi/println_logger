#![deny(missing_docs)]
//! Logging for very lazy. Just println!-s every message of level `info!` and above.
extern crate log;

use std::io::Write;

/// A trivial logger to use before you embrace the actual env_logger.
/// Turns `info!`, `warn!` and `error!` into `println!`.
pub struct PrintlnLogger;
impl log::Log for PrintlnLogger {
    fn enabled(&self, _:&log::Metadata)->bool {true}
    fn log(&self, record:&log::Record) {
        println!("{}", record.args());
    }
    fn flush(&self) {
        let _ = std::io::stdout().flush();
    }
}

static LOGGER: PrintlnLogger = PrintlnLogger;

/// Try to initialize the println_logger, not caring if something failed
pub fn init() {
    let _ = log::set_logger(&LOGGER);
    log::set_max_level(log::LevelFilter::Info);
}
