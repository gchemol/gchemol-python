// [[file:~/Workspace/Programming/pygchemol/pygchemol.note::f9374825-1d77-47ff-92f2-ae6d7eb99207][f9374825-1d77-47ff-92f2-ae6d7eb99207]]
#![feature(proc_macro, specialization)]
extern crate pyo3;
use pyo3::prelude::*;

use pyo3::py::modinit as pymodinit;

// Add bindings to the generated python module
// N.B: names: "librust2py" must be the name of the `.so` or `.pyd` file
/// This module is implemented in Rust.
#[pymodinit(_gchemol)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "hello")]
    // ``#[pyfn()]` converts the arguments from Python objects to Rust values
    // and the Rust return value back into a Python object.
    fn hello() -> PyResult<String> {
        let out = "hello world".into();
        Ok(out)
    }

    Ok(())
}
// f9374825-1d77-47ff-92f2-ae6d7eb99207 ends here
