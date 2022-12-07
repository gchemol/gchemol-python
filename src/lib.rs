// [[file:../pygchemol.note::57ba050b][57ba050b]]
use pyo3::prelude::*;
// 57ba050b ends here

// [[file:../pygchemol.note::8fc5b8be][8fc5b8be]]
use gchemol::prelude::*;
use gchemol::Molecule as GMolecule;

#[pyclass(mapping, module = "gchemol", subclass)]
#[derive(Clone)]
pub struct Molecule {
    inner: GMolecule,
}

#[pymethods]
impl Molecule {
    /// Construct `Molecule` object from a file `path`. If the file
    /// contains multiple molecules, only the last one will be read.
    #[staticmethod]
    fn from_file(py: Python, path: String) -> PyResult<Self> {
        let inner = GMolecule::from_file(&path)?;
        Ok(Self { inner })
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

    /// Get the number of bonds.
    fn nbonds(&self) -> PyResult<usize> {
        let n = self.inner.nbonds();
        Ok(n)
    }

    /// Unbuild current crystal structure leaving a nonperiodic structure
    fn unbuild_crystal(&mut self) -> PyResult<()> {
        self.inner.unbuild_crystal();
        Ok(())
    }

    /// Recalculates all bonds in molecule based on interatomic
    /// distances and covalent radii. For periodic system, the bonds
    /// are determined by applying miniumu image convention.
    fn rebond(&mut self) -> PyResult<()> {
        self.inner.rebond();
        Ok(())
    }
}
// 8fc5b8be ends here

// [[file:../pygchemol.note::ec553356][ec553356]]
#[pymodule]
#[pyo3(name = "gchemol")]
fn pygchemol(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<Molecule>()?;

    Ok(())
}
// ec553356 ends here
