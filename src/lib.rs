// lib.rs
// :PROPERTIES:
// :header-args: :tangle src/lib.rs
// :END:

// [[file:~/Workspace/Programming/gchemol-rs/python/pygchemol.note::*lib.rs][lib.rs:1]]
use octree::Octree;
use pyo3::prelude::*;
use rayon::prelude::*;
use vecfx::*;

type Point = [f64; 3];

// logic implemented as a normal rust function
fn octree_search(points: &[Point], bucket_size: usize, cutoff: f64) -> Vec<Vec<(usize, f64)>> {
    let mut tree = Octree::new(points.to_vec());
    tree.build(bucket_size);

    points
        .par_iter()
        .map(|&q| {
            let x: Vec<_> = tree.search(q, cutoff).collect();
            x
        })
        .collect()
}

// add bindings to the generated python module
// N.B: names: "librust2py" must be the name of the `.so` or `.pyd` file
/// This module is implemented in Rust.
#[pymodule]
fn pygchemol(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // PyO3 aware function. All of our python interfaces could be declared in a separate module.
    // Note that the `#[pyfn()]` annotation automatically converts the arguments from
    // Python objects to Rust values; and the Rust return value back into a Python object.
    /// xxx
    /// Parameters
    /// ----------
    /// - points: List
    /// - pt: list
    #[pyfn(m, "octree_search")]
    fn octree_search_py(_py: Python, points: Vec<f64>) -> PyResult<Vec<Vec<(usize, f64)>>> {
        let points = points.as_3d();
        let bucket_size = 64;
        let cutoff = 3.0;
        let out = octree_search(points, bucket_size, cutoff);
        Ok(out)
    }

    Ok(())
}
// lib.rs:1 ends here
