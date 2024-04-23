use pyo3::{prelude::*, types::PyBool};

#[pyclass(name="Result")]
#[derive(Debug)]
pub struct RsResult {
    pub value: Result<PyObject, PyObject>
}