use std::fs::File;

use pyo3::{
    prelude::*, types::{PyString, PyType},
};

use super::rs_result::RsResult;

#[pyclass]
#[derive(Debug)]
pub struct RsFile {
    file: File
} 

// #[pymethods]
// impl RsFile {
//     #[classmethod]
//     pub fn open(path: &Bound<'_, PyType>) -> RsResult {
//         match File::open(path.to_string()) {
//             Ok(f) => RsResult::Ok { value: f.},
//             Err(e) => todo!(),
//         }
//     }
// }