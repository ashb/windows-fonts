use pyo3::{exceptions::PyOSError, PyErr};

pub struct PyWindowsErr(windows::core::Error);

impl From<windows::core::Error> for PyWindowsErr {
    fn from(err: windows::core::Error) -> Self {
        PyWindowsErr(err)
    }
}

impl std::fmt::Display for PyWindowsErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.message())
    }
}

impl From<PyWindowsErr> for PyErr {
    fn from(err: PyWindowsErr) -> PyErr {
        PyOSError::new_err(err.to_string())
    }
}
