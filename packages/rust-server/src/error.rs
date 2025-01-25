use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use snafu::prelude::*;

#[derive(Snafu, Debug)]
pub(crate) enum MyError {
    #[snafu(visibility(pub))]
    #[snafu(display("serialize/deserialize error: {source} at {loc}"))]
    Serde {
        source: serde_json::Error,
        #[snafu(implicit)]
        loc: snafu::Location,
    },
    #[snafu(visibility(pub))]
    #[snafu(display("Rust common library error: {source} at {loc}"))]
    Common {
        source: common::error::CommonError,
        #[snafu(implicit)]
        loc: snafu::Location,
    },
    #[snafu(visibility(pub))]
    #[snafu(display("Rust/Python conversion error: {source} at {loc}"))]
    Conversion {
        source: pythonize::PythonizeError,
        #[snafu(implicit)]
        loc: snafu::Location,
    },
    #[snafu(visibility(pub))]
    #[snafu(display("Unknown error: {msg} at {loc}"))]
    Unknown {
        msg: String,
        #[snafu(implicit)]
        loc: snafu::Location,
    },
}

impl From<MyError> for PyErr {
    fn from(error: MyError) -> Self {
        match error {
            MyError::Serde { .. } => PyTypeError::new_err(format!("{:?}", error)),
            _ => PyValueError::new_err(format!("{:?}", error)),
        }
    }
}
