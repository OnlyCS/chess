macro_rules! ensure {
    ($condition:expr, $error:expr) => {
        if !$condition {
            return Err($error.into());
        }
    };
}

macro_rules! ensure_eq {
    ($a:expr, $b:expr, $error:expr) => {
        if $a != $b {
            return Err($error.into());
        }
    };
}

pub(crate) use ensure;
pub(crate) use ensure_eq;