use pyo3::{
    exceptions::{PyKeyError, PyOSError, PyRuntimeError},
    PyErr,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WindowsFontError {
    #[error(transparent)]
    WindowsErr(#[from] windows::core::Error),
    #[error("{0}")]
    Windows10Needed(String),

    #[error("{0} doesn't exist")]
    KeyNotFound(String),
}

impl From<WindowsFontError> for PyErr {
    fn from(err: WindowsFontError) -> Self {
        match err {
            WindowsFontError::WindowsErr(e) => PyOSError::new_err(e.to_string()),
            WindowsFontError::Windows10Needed(msg) => PyRuntimeError::new_err(msg),
            WindowsFontError::KeyNotFound(msg) => PyKeyError::new_err(msg),
        }
    }
}

impl From<anyhow::Error> for WindowsFontError {
    fn from(value: anyhow::Error) -> Self {
        match value.downcast::<windows::core::Error>() {
            Ok(win_err) => WindowsFontError::WindowsErr(win_err),
            Err(_) => panic!("argh"),
        }
    }
}
