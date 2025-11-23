mod client;
mod error;
mod types;

use pyo3::prelude::*;

#[pymodule]
fn _ews_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<client::PyEwsClient>()?;
    Ok(())
}
