use log::*;

pub struct SimpleLogger;
pub static LOGGER: SimpleLogger = SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            #[cfg(all(not(target_arch = "wasm32"), not(test)))]
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}