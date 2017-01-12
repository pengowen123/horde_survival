/// Logs the provided message, then panics with a crash message
macro_rules! crash {
    ($msg:tt, $($arg:expr),*) => {{
        error!($msg, $($arg,)*);
        panic!($crate::consts::misc::CRASH_MESSAGE);
    }};
    ($msg:tt) => {{
        crash!($msg,);
    }};
}

/// Unwraps a result, or logs the error and panics
macro_rules! unwrap_pretty {
    ($val:expr) => {{
        match $val {
            Ok(val) => val,
            Err(e) => crash!("{}", e),
        }
    }}
}
