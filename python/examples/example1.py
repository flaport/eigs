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
