# eigs (Rust/Python Package)

Find Eigenvalues and Eigenvectors with Rust/Python using UMFPACK + ARPACK.

## Packages

- Rust Library: `eigs` @ [crates.io](https://crates.io/crates/eigs): `cargo add eigs`
- Python Library: `eigs` @ [pypi.org](https://pypi.org/project/eigs): `pip install eigs` (Linux Only for now)

## Examples

### `eigs` (The Python Package)

```python
import numpy as np
from scipy.sparse import csc_matrix
from eigs import eigs

data = np.array([ 0.+4.j,  2.+3.j,  4.+0.j,  0.-2.j, -3.+3.j,  0.-1.j,
                 -3.-3.j,  4.-4.j, -4.+0.j,  4.+1.j, -4.-1.j,  4.+2.j,
                  3.+2.j,  0.+2.j, -4.+0.j, -4.+1.j, -4.+2.j, -2.+2.j,
                 -1.-2.j,  0.+3.j, -3.+0.j,  4.+0.j, -1.+3.j])  # fmt: skip
indices = np.array([6, 1, 4, 5, 7, 2, 3, 4, 5, 6, 7, 1,
                    1, 3, 3, 4, 0, 2, 5, 6, 3, 6, 7])  # fmt: skip
indptr = np.array([0, 1, 5, 11, 12, 14, 16, 20, 23])
A = csc_matrix((data, indices, indptr))
vals, vecs = eigs(A=A, num_eigs=4, sigma=-2.0 + 7.0j)

for i, val in enumerate(vals):
    print(f"{i}: {val.real} + {val.imag}j")
```
```
0: -2.3310185657008846 + 7.624960781252993j
1: -4.525347075933688 + 1.8131068538310453j
2: 5.301183172745191 + 4.2055904210543575j
3: 0.1713950830265607 + 0.46316839127801934j
```

- [Read More...](python)

### `eigs` (The Rust Package)

```rust
#[allow(non_snake_case)]
fn main() {
    use eigs::eigs::eigs;
    use num_complex::Complex64;
    let k = 4; // number of eigenvalues we want returned
    let n = 8; // nxn matrix A
    let Ap = vec![0, 1, 5, 11, 12, 14, 16, 20, 23];
    let Ai = vec![6, 1, 4, 5, 7, 2, 3, 4, 5, 6, 7, 1, 1, 3, 3, 4, 0, 2, 5, 6, 3, 6, 7];
    let Are = vec![0., 2., 4., 0., -3., 0., -3., 4., -4., 4., -4., 4., 3., 0., -4., -4., -4., -2., -1., 0., -3., 4., -1.];
    let Aim = vec![4., 3., 0., -2., 3., -1., -3., -4., 0., 1., -1., 2., 2., 2., 0., 1., 2., 2., -2., 3., 0., 0., 3.];
    let Ax: Vec<Complex64> = Are.iter().zip(Aim.iter())
        .map(|(re, im)| Complex64 { re: *re, im: *im })
        .collect();
    let sigma = Complex64 { re: -2.0, im: 7.0 };
    let (vals, _) = eigs(k, n, &Ap, &Ai, &Ax, sigma, None);
    for (i, val) in vals.iter().enumerate() {
        println!("{}: {} + {}j", i, val.re, val.im);
    }
}
```
```
0: -2.3310185657008846 + 7.624960781252993j
1: -4.525347075933688 + 1.8131068538310453j
2: 5.301183172745191 + 4.2055904210543575j
3: 0.1713950830265607 + 0.46316839127801934j
```

- [Read More...](rust)


## License & Credits

© Floris Laporte 2023, LGPL-2.1
