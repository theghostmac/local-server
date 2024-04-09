use std::sync::{Mutex, Once};

static INIT: Once = Once::new();
static mut LOGGER: Option<Mutex<Logger>> = None;

pub struct Logger {

}

impl Logger {
    fn new() -> Logger {
        Logger {}
    }

    pub fn log(&self, message: String) {
        println!("{}", message);
    }
}

pub fn global_logger() -> &'static Mutex<Logger> {
    unsafe {
        INIT.call_once(|| {
            LOGGER = Some(Mutex::new(Logger::new()));
        });
        LOGGER.as_ref().unwrap()
    }
}

// Function-based loggers.
pub fn info(msg: &str) {
    global_logger().lock().unwrap().log(msg.to_string());
}

pub fn debug(msg: &str) {
    global_logger().lock().unwrap().log(format!{"DEBUG: {}", msg});
}

// Macro definitions for logging.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::log::logger::info(&format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        $crate::log::logger::debug(&format!($($arg)*));
    }};
}