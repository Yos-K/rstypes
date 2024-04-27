use pyo3::{
    prelude::*,
    types::PyBool
};

#[pyclass(name="Result")]
#[derive(Debug, FromPyObject)]
pub enum RsResult {
    Ok {value: PyObject},
    Err {value: PyObject}
}

#[pymethods]
impl RsResult {
    #[new]
    pub fn new(n: PyObject, t: String) -> PyResult<Self> {
        match t.as_str() {
            "Ok" => Ok(RsResult::Ok { value: n }),
            "Err" => Ok(RsResult::Err { value: n }),
            _ => panic!("")
        }
    }

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
            RsResult::Ok { value } => todo!(),
            RsResult::Err { value } => todo!(),
        }
    }
}