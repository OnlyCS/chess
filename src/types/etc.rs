use std::error::Error;

pub trait ToResult<T> {
    fn to_result(self, error: Box<dyn Error>) -> Result<T, Box<dyn Error>>;
}

impl<T> ToResult<T> for Option<T> {
    fn to_result(self, error: Box<dyn Error>) -> Result<T, Box<dyn Error>> {
        match self {
            Some(t) => Ok(t),
            None => Err(error),
        }
    }
}
