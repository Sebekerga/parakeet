macro_rules! debug {
    ($ticket_id:expr, $($arg:tt)*) => {
        log::debug!("[{}] {}", $ticket_id, format!($($arg)*))
    };
}

macro_rules! info {
    ($ticket_id:expr, $($arg:tt)*) => {
        log::info!("[{}] {}", $ticket_id, format!($($arg)*))
    };
}

macro_rules! warng {
    ($ticket_id:expr, $($arg:tt)*) => {
        log::warn!("[{}] {}", $ticket_id, format!($($arg)*))
    };
}

macro_rules! error {
    ($ticket_id:expr, $($arg:tt)*) => {
        log::error!("[{}] {}", $ticket_id, format!($($arg)*))
    };
}

pub(crate) use debug;
pub(crate) use error;
pub(crate) use info;
pub(crate) use warng;
