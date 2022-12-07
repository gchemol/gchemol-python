// [[file:../pygchemol.note::57ba050b][57ba050b]]
use pyo3::prelude::*;
// 57ba050b ends here

// [[file:../pygchemol.note::8fc5b8be][8fc5b8be]]
use gchemol::prelude::*;
use gchemol::Molecule as GMolecule;
use gut::prelude::*;

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

    /// Return its json representation of molecule object.
    fn to_json(&self) -> PyResult<String> {
        let json = gchemol::io::to_json(&self.inner)?;
        Ok(json)
    }

    /// Write molecule to file with `path`. The molecule format will
    /// be determined based on file name extension.
    fn to_file(&self, path: String) -> PyResult<()> {
        self.inner.to_file(&path)?;
        Ok(())
    }

    /// Render molecule with template file from `path`. On success,
    /// return the formatted string.
    fn render_with(&self, path: String) -> PyResult<String> {
        let s = self.inner.render_with(path.as_ref())?;
        Ok(s)
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

    /// Create a Lattice from the minimal bounding box of the Molecule
    /// extended by a positive value of padding.
    ///
    /// NOTE: padding has to be large enough (> 0.5) to avoid self
    /// interaction with its periodic mirror.
    fn set_lattice_from_bounding_box(&mut self, padding: f64) -> PyResult<()> {
        self.inner.set_lattice_from_bounding_box(padding);
        Ok(())
    }

    /// Create a supercell version of new molecule.
    ///
    /// # Arguments
    ///
    /// * sa, sb, sc: an sequence of three scaling factors. E.g., [2,
    /// 1, 1] specifies that the supercell should have dimensions 2a x
    /// b x c
    fn supercell(&mut self, sa: usize, sb: usize, sc: usize) -> PyResult<Self> {
        if let Some(mol) = self.inner.supercell(sa, sb, sc) {
            let m = Self { inner: mol };
            Ok(m)
        } else {
            Err(pyo3::exceptions::PyException::new_err("not allowed for periodic structure"))
        }
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
