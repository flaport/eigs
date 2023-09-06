use eigs_rs;
use num_complex::Complex64;
use pyo3::prelude::{pyfunction, pymodule, PyModule, PyResult, Python};
use pyo3::wrap_pyfunction;
use umfpack::complex::as_float_array;

pub mod read;

#[pymodule]
fn eigs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_eigs, m)?)?;
    Ok(())
}

#[pyfunction]
#[allow(non_snake_case)]
pub fn _eigs(
    num_eigs: i32,
    n: i32,
    Ap_bytes: Vec<u8>,
    Ai_bytes: Vec<u8>,
    Az_bytes: Vec<u8>,
    sigma_r: f64,
    sigma_i: f64,
) -> (Vec<f64>, Vec<f64>) {
    let Ap: Vec<i32> = read::parse_i32_vec(&Ap_bytes);
    let Ai: Vec<i32> = read::parse_i32_vec(&Ai_bytes);
    let Az: Vec<Complex64> = read::parse_c64_vec(&Az_bytes);
    let sigma = Complex64 {
        re: sigma_r,
        im: sigma_i,
    };
    let (vals, vecs) = eigs_rs::eigs::eigs(num_eigs, n, &Ap, &Ai, &Az, sigma, None);
    return (
        as_float_array(&vals).iter().map(|f| *f).collect(),
        as_float_array(&vecs).iter().map(|f| *f).collect(),
    );
}
