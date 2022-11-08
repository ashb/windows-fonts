use pyo3::{
    exceptions::{PyOSError, PyRuntimeError},
    PyErr,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WindowsFontError {
    #[error(transparent)]
    WindowsErr(#[from] windows::core::Error),
    #[error("{0}")]
    Windows10Needed(String),
}

impl From<WindowsFontError> for PyErr {
    fn from(err: WindowsFontError) -> Self {
        match err {
            WindowsFontError::WindowsErr(e) => PyOSError::new_err(e.to_string()),
            WindowsFontError::Windows10Needed(msg) => PyRuntimeError::new_err(msg),
        }
    }
}
