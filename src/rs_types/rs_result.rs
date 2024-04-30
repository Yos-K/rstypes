use pyo3::{
    prelude::*,
    types::PyBool
};

use super::rs_option::RsOption;

#[pyclass(name="Result")]
#[derive(Debug, FromPyObject)]
pub enum RsResult {
    Ok {value: PyObject},
    Err {value: PyObject}
}

#[pymethods]
impl RsResult {
    fn __str__(&self) -> String {
        match &self {
            RsResult::Ok { value } => format!("Ok({})", &value),
            RsResult::Err { value } => format!("Err({})", &value),
        }
    }

    pub const fn is_ok(&self) -> bool {
        match self {
            RsResult::Ok { value: _ } => true,
            RsResult::Err { value: _ } => false,
        }
    }

    pub fn is_ok_and(&self, f: PyObject) -> bool {
        match &self {
            RsResult::Err { value: _ } => false,
            RsResult::Ok { value: x } => {
                Python::with_gil(|py| {
                    f.call1(py, (x,)).map(|r| r.to_object(py)).unwrap()
                    .downcast_bound::<PyBool>(py).unwrap().extract().unwrap()
                })
            },
        }
    }

    pub const fn is_err(&self) -> bool {
        !self.is_ok()
    }

    pub fn is_err_and(&self, f: PyObject) -> bool {
        match self {
            RsResult::Ok { value: _ } => false,
            RsResult::Err { value: x } => {
                Python::with_gil(|py| {
                    f.call1(py, (x,)).map(|r| r.to_object(py)).unwrap()
                    .downcast_bound::<PyBool>(py).unwrap().extract().unwrap()
                })
            },
        }
    }

    pub fn ok(&self) -> RsOption {
        match &self {
            RsResult::Ok { value } => RsOption::new(Some(value.clone())),
            RsResult::Err { value: _ } => RsOption::new(None),
        }
    }

    pub fn err(&self) -> RsOption {
        match &self {
            RsResult::Ok { value: _ } => RsOption::new(None),
            RsResult::Err { value } => RsOption::new(Some(value.clone())),
        }
    }

    pub fn map(&self, f: PyObject) -> RsResult {
        match &self {
            RsResult::Ok { value } => {
                RsResult::Ok { value: Python::with_gil(|py| {
                    f.call1(py, (value,))
                }).unwrap() }
            },
            RsResult::Err { value } => RsResult::Err { value: value.clone() },
        }
    }

    pub fn map_or(&self, default: PyObject, f: PyObject) -> PyObject {
        match &self {
            RsResult::Ok { value } => {
                Python::with_gil(|py| {
                    f.call1(py, (value,))
                }).unwrap()
            },
            RsResult::Err { value: _ } => default,
        }
    }

    pub fn map_or_else(&self, default: PyObject, f: PyObject) -> PyObject {
        match &self {
            RsResult::Ok { value } => {
                Python::with_gil(|py| f.call1(py, (value,))).unwrap()
            },
            RsResult::Err { value: _ } => {
                Python::with_gil(|py| default.call0(py)).unwrap()
            },
        }
    }

    pub fn map_err(&self, op: PyObject) -> RsResult {
        match &self {
            RsResult::Ok { value } => RsResult::Ok { value: value.clone() },
            RsResult::Err { value } => {
                RsResult::Err { value: Python::with_gil(|py| {
                    op.call1(py, (value,))
                }).unwrap() }
            },
        }
    }
}