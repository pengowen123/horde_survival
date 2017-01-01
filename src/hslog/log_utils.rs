macro_rules! unwrap_or_log {
    ($val:expr, $msg:tt) => {{
        unwrap_or_log!($val, $msg,)
    }};

    ($val:expr, $msg:tt, $($arg:expr),*) => {{
        if $val.can_unwrap() {
            $val.unwrap()
        } else {
            crash!($msg, $($arg)*);
        }
    }};
}

macro_rules! crash {
    ($msg:tt, $($arg:expr),*) => {{
        error!($msg, $($arg)*);
        panic!($crate::consts::misc::CRASH_MESSAGE);
    }};
}

macro_rules! unwrap_pretty {
    ($val:expr) => {{
        match $val {
            Ok(val) => val,
            Err(e) => crash!("{}", e),
        }
    }}
}

pub trait CanUnwrap {
    fn can_unwrap(&self) -> bool;
}

impl<T> CanUnwrap for Option<T> {
    fn can_unwrap(&self) -> bool {
        self.is_some()
    }
}

impl<T, E> CanUnwrap for Result<T, E> {
    fn can_unwrap(&self) -> bool {
        self.is_ok()
    }
}
