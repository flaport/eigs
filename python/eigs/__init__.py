__author__ = "Floris Laporte"
__version__ = "0.0.1"

import numpy as np
from scipy.sparse import csc_matrix

from eigs.eigs import _eigs # pyo3 lib


def eigs(
    A: csc_matrix,
    num_eigs: int,
    sigma: complex,
):
    m, n = A.shape
    if m != n:
        raise ValueError("A is not square!")
    vals, vecs = _eigs(
        num_eigs=round(num_eigs),
        n=round(n),
        Ap_bytes=np.asarray(A.indptr, dtype=np.int32).tobytes(),
        Ai_bytes=np.asarray(A.indices, dtype=np.int32).tobytes(),
        Az_bytes=np.asarray(A.data, dtype=np.complex128).tobytes(),
        sigma_r=np.float64(sigma.real),
        sigma_i=np.float64(sigma.imag),
    )
    vals = np.asarray(vals, dtype=np.float64).view(np.complex128)
    vecs = np.asarray(vecs, dtype=np.float64).view(np.complex128)
    return vals, vecs.reshape(-1, n)[:num_eigs].T
