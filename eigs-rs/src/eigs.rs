use super::csc::subtract_sparse_csc_matrix;
use super::split::xslice_yslice;
use arpack_ng_sys::{__BindgenComplex, znaupd_c, zneupd_c};
use num_complex::Complex64;
use umfpack::numeric::Numeric;
use umfpack::symbolic::Symbolic;
use umfpack::sys::UMFPACK;
use umfpack::zi::{umfpack_zi_numeric, umfpack_zi_solve, umfpack_zi_symbolic};

pub struct EigsConfig {
    pub howmny: String,
    pub bmat: String,
    pub ishfts: i32,
    pub maxiter: Option<i32>,
    pub mode: i32,
    pub tol: f64,
    pub which: String,
}

impl EigsConfig {
    pub fn new() -> Self {
        return Self {
            howmny: "A".into(),
            bmat: "I".into(),
            ishfts: 1,
            maxiter: None,
            mode: 3,
            tol: 1e-6,
            which: "LM".into(),
        };
    }
}

#[allow(non_snake_case)]
pub fn eigs(
    num_eigs: i32,
    n: i32,
    Ap: &[i32],
    Ai: &[i32],
    Az: &[Complex64],
    sigma: Complex64,
    config: Option<EigsConfig>,
) -> (Vec<Complex64>, Vec<Complex64>) {
    let return_eigenvectors = true;
    let config = match config {
        None => EigsConfig::new(),
        Some(config) => config,
    };
    let mut ido = 0;
    let mut info = 0;
    let maxiter = match config.maxiter {
        None => 100 * n,
        Some(maxiter) => maxiter,
    };
    let ncv = i32::min(i32::max(2 * num_eigs + 1, 20), n);
    let lworkl = 3 * ncv * (ncv + 2);
    let Sp: Vec<i32> = (0..n + 1).map(|i| i).collect();
    let Si: Vec<i32> = (0..n).map(|i| i).collect();
    let Sz: Vec<Complex64> = (0..n).map(|_| sigma).collect();
    let (Asigp, Asigi, Asigz) = subtract_sparse_csc_matrix(&Ap, &Ai, &Az, &Sp, &Si, &Sz);

    let mut symbolic = Symbolic::new();
    umfpack_zi_symbolic(n, n, &Asigp, &Asigi, &Asigz, &mut symbolic, None, None);

    let mut numeric = Numeric::new();
    umfpack_zi_numeric(&Asigp, &Asigi, &Asigz, &symbolic, &mut numeric, None, None);

    let mut resid: Vec<Complex64> = (0..n).map(|_| Complex64 { re: 0.0, im: 0.0 }).collect();
    let mut v: Vec<Complex64> = (0..n * ncv)
        .map(|_| Complex64 { re: 0.0, im: 0.0 })
        .collect();
    let mut iparam = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    iparam[0] = config.ishfts;
    iparam[2] = maxiter;
    iparam[3] = 1;
    iparam[6] = config.mode;

    let mut workd: Vec<Complex64> = (0..3 * n).map(|_| Complex64 { re: 0.0, im: 0.0 }).collect();
    let mut workl: Vec<Complex64> = (0..lworkl)
        .map(|_| Complex64 { re: 0.0, im: 0.0 })
        .collect();
    let mut rwork: Vec<f64> = (0..ncv).map(|_| 0.0).collect(); // real
    let mut ipntr: Vec<i32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    loop {
        unsafe {
            znaupd_c(
                &mut ido,
                (&config.bmat).as_ptr() as *const i8,
                n as i32,
                (&config.which).as_ptr() as *const i8,
                num_eigs as i32,
                config.tol as f64,
                (&mut resid).as_mut_ptr() as *mut __BindgenComplex<f64>,
                ncv as i32,
                (&mut v).as_mut_ptr() as *mut __BindgenComplex<f64>,
                n as i32,
                (&mut iparam).as_mut_ptr(),
                (&mut ipntr).as_mut_ptr(),
                (&mut workd).as_mut_ptr() as *mut __BindgenComplex<f64>,
                (&mut workl).as_mut_ptr() as *mut __BindgenComplex<f64>,
                lworkl as i32,
                (&mut rwork).as_mut_ptr(),
                &mut info,
            );
        }
        let n0 = ipntr[0] - 1;
        let n1 = ipntr[1] - 1;
        let n2 = ipntr[2] - 1;

        match ido {
            -1 => {
                // initialization
                let (xslice, yslice) = xslice_yslice(&mut workd, n0, n1, n);
                umfpack_zi_solve(
                    UMFPACK::A,
                    &Asigp,
                    &Asigi,
                    &Asigz,
                    yslice,
                    xslice,
                    &numeric,
                    None,
                    None,
                );
            }
            1 => {
                let (xslice, yslice) = xslice_yslice(&mut workd, n2, n1, n);
                umfpack_zi_solve(
                    UMFPACK::A,
                    &Asigp,
                    &Asigi,
                    &Asigz,
                    yslice,
                    xslice,
                    &numeric,
                    None,
                    None,
                );
            }
            2 => {
                for i in 0..n as usize {
                    let (xslice, yslice) = xslice_yslice(&mut workd, n0, n1, n);
                    yslice[i] = xslice[i]
                }
            }
            3 => {
                panic!("ARPACK requested user shifts. Assure ISHIFT==0");
            }
            _ => {
                break;
            }
        }
    }
    if info != 0 {
        panic!("ARPACKERROR");
    }

    let sselect: Vec<i32> = (0..ncv).map(|_| 0).collect();
    let mut workev: Vec<Complex64> = (0..3 * ncv)
        .map(|_| Complex64 { re: 0.0, im: 0.0 })
        .collect();
    let mut vals: Vec<Complex64> = (0..num_eigs)
        .map(|_| Complex64 { re: 0.0, im: 0.0 })
        .collect();
    let mut vecs: Vec<Complex64> = (0..n * ncv)
        .map(|_| Complex64 { re: 0.0, im: 0.0 })
        .collect();

    unsafe {
        let _sigma = __BindgenComplex {
            re: sigma.re,
            im: sigma.im,
        };
        zneupd_c(
            return_eigenvectors as i32,
            config.howmny.as_ptr() as *const i8,
            (&sselect).as_ptr() as *const i32,
            (&mut vals).as_mut_ptr() as *mut __BindgenComplex<f64>,
            (&mut vecs).as_mut_ptr() as *mut __BindgenComplex<f64>,
            n as i32,
            _sigma,
            (&mut workev).as_mut_ptr() as *mut __BindgenComplex<f64>,
            (&config.bmat).as_ptr() as *const i8,
            n as i32,
            (&config.which).as_ptr() as *const i8,
            num_eigs as i32,
            config.tol as f64,
            (&mut resid).as_mut_ptr() as *mut __BindgenComplex<f64>,
            ncv as i32,
            (&mut v).as_mut_ptr() as *mut __BindgenComplex<f64>,
            n as i32,
            (&mut iparam).as_mut_ptr() as *mut i32,
            (&mut ipntr).as_mut_ptr() as *mut i32,
            (&mut workd).as_mut_ptr() as *mut __BindgenComplex<f64>,
            (&mut workl).as_mut_ptr() as *mut __BindgenComplex<f64>,
            lworkl as i32,
            (&mut rwork).as_mut_ptr() as *mut f64,
            &mut info,
        );
    }

    if info != 0 {
        panic!("ARPACKERROR");
    }

    return (vals, vecs);
}
