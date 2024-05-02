mod rs_types;

use pyo3::prelude::*;
use rs_types::{
    rs_option::RsOption,
    rs_result::RsResult,
    rs_file::RsFile
};



#[pymodule]
fn rstypes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RsOption>()?;
    m.add_class::<RsResult>()?;
    m.add_class::<RsFile>()?;
    Ok(())
}
