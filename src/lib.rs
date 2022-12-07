// [[file:../pygchemol.note::57ba050b][57ba050b]]
use pyo3::prelude::*;
// 57ba050b ends here

// [[file:../pygchemol.note::8fc5b8be][8fc5b8be]]
use gchemol::prelude::*;
use gchemol::Molecule;

#[pyclass(mapping, module = "pygchemol", subclass)]
// #[pyo3(text_signature = "(/, multigraph=True, attrs=None)")]
#[derive(Clone)]
pub struct PyMolecule {
    inner: Molecule,
    // #[pyo3(get, set)]
    // pub attrs: PyObject,
}

#[pymethods]
impl PyMolecule {
    // #[new]
    // // NOTE: set default python argument
    // // #[args(multigraph = "true")]
    // fn new(py: Python, attrs: Option<PyObject>) -> Self {
    //     Self {
    //         attrs: attrs.unwrap_or_else(|| py.None()),
    //     }
    // }

    /// Construct `Molecule` object from a file `path`. If the file
    /// contains multiple molecules, only the last one will be read.
    #[staticmethod]
    fn from_file(py: Python, path: String) -> PyResult<Self> {
        let inner = Molecule::from_file(&path)?;
        Ok(Self { inner })
    }

    /// Add a new node to the graph.
    ///
    /// :param obj: The python object to attach to the node
    ///
    /// :returns: The index of the newly created node
    /// :rtype: int
    #[pyo3(text_signature = "(self, obj, /)")]
    fn add_atom(&mut self, obj: PyObject) -> PyResult<()> {
        println!("add one node");
        Ok(())
    }

    /// Dump molecule object in json format.
    fn to_json(&self) -> PyResult<()> {
        let json = gchemol::io::to_json(&self.inner)?;
        println!("{json}");
        Ok(())
    }

    /// Get the number of atoms.
    fn natoms(&self) -> PyResult<usize> {
        let n = self.inner.natoms();
        Ok(n)
    }

    /// Return chemical formula.
    fn formula(&self) -> PyResult<String> {
        Ok(self.inner.formula())
    }

    /// Unbuild current crystal structure leaving a nonperiodic structure
    fn unbuild_crystal(&self) -> PyResult<()> {
        self.unbuild_crystal();
        Ok(())
    }

    /// Get the number of bonds.
    fn nbonds(&self) -> PyResult<usize> {
        let n = self.inner.nbonds();
        Ok(n)
    }
}
// 8fc5b8be ends here

// [[file:../pygchemol.note::ec553356][ec553356]]
#[pymodule]
fn pygchemol(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<PyMolecule>()?;

    Ok(())
}
// ec553356 ends here
