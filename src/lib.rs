extern crate pyo3;
use pyo3::prelude::*;
use pyo3::wrap_pymodule;

mod client;
mod server;
use server::*;
use client::*;

#[pymodule]
fn not_alone(_p: Python, m: &PyModule) -> PyResult<()>{
    m.add_wrapped(wrap_pymodule!(server))?;
    m.add_wrapped(wrap_pymodule!(client))?;

    Ok(())
}


