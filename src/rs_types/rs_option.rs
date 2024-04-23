use pyo3::{prelude::*, types::PyBool};

use super::rs_result::RsResult;

#[pyclass(name="Option")]
#[derive(Debug)]
pub struct RsOption {
    value: Option<PyObject>
}

#[pymethods]
impl RsOption {
    #[new]
    pub fn new(n: Option<PyObject>) -> Self {
        Self { value: n }
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

    // fn as_slice(&self) -> PyResult<PyList> {
    //     let a = Python::with_gil(|py| {
    //         PyList::new_bound(py, self.value.as_slice())
    //     }).extract::<PyList>();
    // }

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

    fn ok_or(&self, err: PyObject) -> RsResult {
        match &self.value {
            Some(v) => RsResult{value: Ok(v.clone())},
            None => RsResult{ value: Err(err) },
        }
    }
}