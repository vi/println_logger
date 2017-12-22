#![deny(missing_docs)]
//! Logging for very lazy. Just println!-s every message of level `info!` and above.
extern crate log;


/// A tribvial logger to use before you embrace the actual env_logger.
/// Turns `info!`, `warn!` and `error!` into `println!`.
pub struct PrintlnLogger;
impl log::Log for PrintlnLogger {
    fn enabled(&self, _:&log::LogMetadata)->bool {true}
    fn log(&self, record:&log::LogRecord) {
        println!("{}", record.args());
    }
}

/// Try to initialize the println_logger, not caring if something failed
pub fn init() {
    let _ = log::set_logger(|m| {
        m.set(log::LogLevelFilter::Info); 
        Box::new(PrintlnLogger{})
    });
}
