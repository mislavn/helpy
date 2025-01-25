use snafu::prelude::*;
use std::sync::PoisonError;

#[derive(Snafu, Debug)]
pub enum CommonError {
    #[snafu(display("Invalid data Error: {msg} at {loc}"))]
    Invalid {
        msg: String,
        #[snafu(implicit)]
        loc: snafu::Location,
    },
    #[snafu(display("Locking error {msg} at {loc}"))]
    Locking {
        msg: String,
        #[snafu(implicit)]
        loc: snafu::Location,
    },
}

impl<T> From<PoisonError<T>> for CommonError {
    fn from(err: PoisonError<T>) -> Self {
        CommonError::Locking {
            msg: format!("Mutex was poisoned: {:?}", err),
            loc: snafu::location!(),
        }
    }
}
