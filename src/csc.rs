use num_complex::Complex64;

pub fn subtract_sparse_csc_matrix(
    ap: &[i32],
    ai: &[i32],
    ax: &[Complex64],
    bp: &[i32],
    bi: &[i32],
    bx: &[Complex64],
) -> (Vec<i32>, Vec<i32>, Vec<Complex64>) {
    let mut cp: Vec<i32> = vec![0; ap.len()];
    let mut ci: Vec<i32> = vec![];
    let mut cx: Vec<Complex64> = vec![];
    let mut i = 0;
    let zero = Complex64 { re: 0.0, im: 0.0 };
    while i < ap.len() - 1 {
        let mut j = ap[i] as usize;
        let mut k = bp[i] as usize;
        while j < ap[i + 1] as usize && k < bp[i + 1] as usize {
            if ai[j] as usize == bi[k] as usize {
                let diff = ax[j] - bx[k];
                if diff != zero {
                    ci.push(ai[j]);
                    cx.push(diff);
                }
                j += 1;
                k += 1;
            } else if ai[j] < bi[k] {
                ci.push(ai[j]);
                cx.push(ax[j]);
                j += 1;
            } else {
                ci.push(bi[k]);
                cx.push(-bx[k]);
                k += 1;
            }
        }
        while j < ap[i + 1] as usize {
            ci.push(ai[j]);
            cx.push(ax[j]);
            j += 1;
        }
        while k < bp[i + 1] as usize {
            ci.push(bi[k]);
            cx.push(-bx[k]);
            k += 1;
        }
        cp[i + 1] = ci.len() as i32;
        i += 1;
    }
    return (cp, ci, cx);
}
