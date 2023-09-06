import numpy as np
from eigs.eigs import _eigs

k = 4  # number of eigenvalues we want returned
n = 8  # nxn matrix A
Ap = np.array([0, 1, 5, 11, 12, 14, 16, 20, 23])
Ai = np.array([6, 1, 4, 5, 7, 2, 3, 4, 5, 6, 7, 1, 1, 3, 3, 4, 0, 2, 5, 6, 3, 6, 7])
Are = np.array([0., 2., 4., 0., -3., 0., -3., 4., -4., 4., -4., 4., 3., 0., -4., -4., -4., -2., -1., 0., -3., 4., -1.])  # fmt: skip
Aim = np.array([4., 3., 0., -2., 3., -1., -3., -4., 0., 1., -1., 2., 2., 2., 0., 1., 2., 2., -2., 3., 0., 0., 3.])  # fmt: skip
Az = Are + 1j * Aim
sigma = -2.0 + 7.0j

vals, vecs = _eigs(
    num_eigs=k,
    n=n,
    Ap_bytes=np.asarray(Ap, dtype=np.int32).tobytes(),
    Ai_bytes=np.asarray(Ai, dtype=np.int32).tobytes(),
    Az_bytes=np.asarray(Az, dtype=np.complex128).tobytes(),
    sigma_r=np.float64(sigma.real),
    sigma_i=np.float64(sigma.imag),
)

vals = np.asarray(vals, dtype=np.float64).view(np.complex128)
vecs = np.asarray(vecs, dtype=np.float64).view(np.complex128).reshape(-1, n)[:k].T

for i, val in enumerate(vals):
    print(f"{i}: {val.real} + {val.imag}j")

# print(vecs)
