macro_rules! safe_unwrap_err {
    ($function:expr, $error:expr) => {
        match $function {
            Ok(value) => value,
            Err(_) => return Err($error.into()),
        }
    };

    ($function:expr) => {
        match $function {
            Ok(value) => value,
            Err(_) => return None,
        }
    };
}

macro_rules! safe_unwrap_option {
    ($function:expr, $error:expr) => {
        match $function {
            Some(value) => value,
            None => return Err($error.into()),
        }
    };

    ($function:expr) => {
        match $function {
            Some(value) => value,
            None => return None,
        }
    };
}

pub(crate) use safe_unwrap_err;
pub(crate) use safe_unwrap_option;
