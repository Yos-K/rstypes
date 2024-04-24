use pyo3::prelude::*;

#[pyclass(name="Result")]
#[derive(Debug, FromPyObject)]
pub enum RsResult {
    Ok {value: PyObject},
    Err {value: PyObject}
}