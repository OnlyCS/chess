use std::{error::Error, fmt::Display};

pub enum UIError {
    CreateFailed,
    DestroyFailed,
}

impl Display for UIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UIError::CreateFailed => "Failed to create UI",
                UIError::DestroyFailed => "Failed to destroy UI",
            }
        )
    }
}

impl From<UIError> for String {
    fn from(val: UIError) -> Self {
        val.to_string()
    }
}

impl From<UIError> for Box<dyn Error> {
    fn from(val: UIError) -> Self {
        val.to_string().into()
    }
}
