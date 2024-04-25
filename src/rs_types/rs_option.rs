use pyo3::{
    prelude::*,
    types::PyBool
};

use super::rs_result::RsResult;

#[pyclass(name="Option")]
#[derive(Debug, FromPyObject)]
pub struct RsOption {
    pub value: Option<PyObject>
}

#[pymethods]
impl RsOption {
    #[new]
    pub fn new(n: Option<PyObject>) -> Self {
        Self { value: n }
    }

    #[getter]
    fn value(&self) -> PyResult<Option<PyObject>> {
        Ok(self.value.clone())
    }

    fn __str__(&self) -> String {
        let s = Python::with_gil(|py| {
            self.value().unwrap().to_object(py).to_string()
        });
        if &s != "None" {
            format!("Some({})", &s)
        } else {
            s
        }
    }

    pub const fn is_some(&self) -> bool {
        self.value.is_some()
    }

    pub fn is_some_and(&self, f: PyObject) -> bool {
        match &self.value {
            None => false,
            Some(x) => {
                Python::with_gil(|py| {
                    f.call1(py, (x,)).map(|r| r.to_object(py)).unwrap()
                    .downcast_bound::<PyBool>(py).unwrap().extract().unwrap()
                })
            },
        }
    }

    pub const fn is_none(&self) -> bool {
        self.value.is_none()
    }

    pub fn expect(&self, msg: String) -> PyObject {
        match &self.value {
            Some(val) => val.clone(),
            None => panic!("{}", &msg),
        }
    }

    pub fn unwrap(&self) -> Option<PyObject> {
        self.value.clone()
    }

    pub fn unwrap_or(&self, default: PyObject) -> PyObject {
        self.value.clone().unwrap_or(default)
    }

    pub fn unwrap_or_else(&self, f: PyObject) -> PyObject {
        match &self.value {
            Some(x) => x.clone(),
            None => Python::with_gil(|py| f.call0(py)).unwrap(),
        }
    }
    
    pub fn map(&self, f: PyObject) -> RsOption {
        RsOption::new(self.value.clone().map(|n| Python::with_gil(|py| {
            f.call1(py, (n,)).map(|r| r.to_object(py)).unwrap()
        })))
    }

    pub fn inspect(&self, f: PyObject) -> Self {
        if let Some(ref x) = self.value {
            Python::with_gil(|py| f.call1(py, (x,))).unwrap();
        }
        Self { value: self.value.clone() }
    }

    pub fn map_or(&self, default: PyObject, f: PyObject) -> PyObject {
        match &self.value {
            Some(t) => {
                Python::with_gil(|py| f.call1(py, (t,))).unwrap()
            },
            None => default,
        }
    }

    pub fn map_or_else(&self, default: PyObject, f: PyObject) -> PyObject {
        match &self.value {
            Some(t) => {
                Python::with_gil(|py| f.call1(py, (t,))).unwrap()
            },
            None => {
                Python::with_gil(|py| default.call0(py)).unwrap()
            },
        }
    }

    pub fn ok_or(&self, err: PyObject) -> RsResult {
        match &self.value {
            Some(v) => RsResult::Ok { value: v.clone() },
            None => RsResult::Err { value: err },
        }
    }

    pub fn ok_or_else(&self, err: PyObject) -> RsResult {
        match &self.value {
            Some(v) => RsResult::Ok { value: v.clone() },
            None => {
                let e = Python::with_gil(|py| {
                    err.call0(py).unwrap()
                });
                RsResult::Err { value: e }
            },
        }
    }

    pub fn and_then(&self, f: PyObject) -> RsOption {
        match &self.value {
            Some(x) => {RsOption::new(Some(Python::with_gil(
                |py| f.call1(py, (x,)).unwrap()
            )))},
            None => RsOption::new(None),
        }
    }

    pub fn or_else(&self, f: PyObject) -> RsOption {
        match &self.value {
            x @ Some(_) => RsOption::new(x.clone()),
            None => {RsOption::new(Some(Python::with_gil(
                |py| f.call0(py).unwrap()
            )))},
        }
    }

    pub fn zip(&self, other: RsOption) -> RsOption {
        match (self.value().unwrap(), other.value().unwrap()) {
            (Some(a), Some(b)) => RsOption::new(Some(
                Python::with_gil(|py| (a, b).to_object(py))
            )),
            _ => RsOption::new(None),
        }
    }
}
