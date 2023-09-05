#[allow(non_snake_case)]
fn main() {
    use eigs::eigs::eigs;
    use num_complex::Complex64;
    let k = 4; // number of eigenvalues we want returned
    let n = 8; // nxn matrix A
    let Ap = vec![0, 1, 5, 11, 12, 14, 16, 20, 23];
    let Ai = vec![
        6, 1, 4, 5, 7, 2, 3, 4, 5, 6, 7, 1, 1, 3, 3, 4, 0, 2, 5, 6, 3, 6, 7,
    ];
    let Are = vec![
        0., 2., 4., 0., -3., 0., -3., 4., -4., 4., -4., 4., 3., 0., -4., -4., -4., -2., -1., 0.,
        -3., 4., -1.,
    ];
    let Aim = vec![
        4., 3., 0., -2., 3., -1., -3., -4., 0., 1., -1., 2., 2., 2., 0., 1., 2., 2., -2., 3., 0.,
        0., 3.,
    ];
    let Ax: Vec<Complex64> = Are
        .iter()
        .zip(Aim.iter())
        .map(|(re, im)| Complex64 { re: *re, im: *im })
        .collect();
    let sigma = Complex64 { re: -2.0, im: 7.0 };
    let (vals, _) = eigs(k, n, &Ap, &Ai, &Ax, sigma, None);
    for (i, val) in vals.iter().enumerate() {
        println!("{}: {} + {}j", i, val.re, val.im);
    }
}
