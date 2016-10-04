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

macro_rules! unwrap_or_log {
    ($val:expr, $msg:tt) => {
        unwrap_or_log!($val, $msg,)
    };
    ($val:expr, $msg:expr, $($arg:expr),*) => {{
        if $val.can_unwrap() {
            $val.unwrap()
        } else {
            crash!($msg, $($arg)*);
        }
    }};
}

macro_rules! crash {
    ($msg:expr, $($arg:expr),*) => {{
        error!($msg, $($arg)*);
        panic!($crate::consts::misc::CRASH_MESSAGE);
    }};
    ($msg:expr) => {{
        error!("{}", $msg);
        panic!($crate::consts::misc::CRASH_MESSAGE);
    }};
}
