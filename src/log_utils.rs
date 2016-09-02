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
    ($val:expr, $msg:tt) => {{
        if $val.can_unwrap() {
            $val.unwrap()
        } else {
            crash!($msg);
        }
    }}
}

macro_rules! crash {
    ($msg:tt) => {{
        error!($msg);
        panic!($crate::consts::misc::CRASH_MESSAGE);
    }};
    ($msg:tt, $($arg:tt),*) => {{
        error!($msg, $($arg)*);
        panic!($crate::consts::misc::CRASH_MESSAGE);
    }};
}
