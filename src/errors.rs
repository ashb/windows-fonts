use pyo3::{exceptions::PyOSError, PyErr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WindowsFontError {
    #[error(transparent)]
    WindowsErr(#[from] windows::core::Error),
}

impl From<WindowsFontError> for PyErr {
    fn from(err: WindowsFontError) -> Self {
        match err {
            WindowsFontError::WindowsErr(e) => PyOSError::new_err(e.to_string()),
        }
    }
}
