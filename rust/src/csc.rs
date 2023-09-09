use num_complex::Complex64;

pub fn csc_subtract(
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

#[allow(non_snake_case)]
pub fn coo_to_csc(
    Ai: &[i32],
    Aj: &[i32],
    Ax: &[Complex64],
) -> (Vec<i32>, Vec<i32>, Vec<Complex64>) {
    // we could in principle split this up in two
    // functions but let's keep it as one for now

    // analyze function:

    let n_nz = Ai.len() as i32;
    let n_col = Aj.iter().fold(0, |max, &x| i32::max(x, max)) + 1;
    let mut Bi = vec![0; n_nz as usize];
    let mut Bk = vec![0; n_nz as usize];
    let mut Bp = vec![0; n_col as usize + 1];

    for n in 0..(n_nz as usize) {
        Bp[Aj[n] as usize] += 1;
    }

    // cumsum the n_nz per row to get Bp
    let mut temp: i32;
    let mut cumsum: i32 = 0;
    for j in 0..(n_col as usize) {
        temp = Bp[j];
        Bp[j] = cumsum;
        cumsum += temp;
    }

    // write Ai, Ax into Bi, Bk
    let mut col: i32;
    let mut dest: usize;
    for n in 0..(n_nz as usize) {
        col = Aj[n];
        dest = Bp[col as usize] as usize;
        Bi[dest] = Ai[n];
        Bk[dest] = n as i32;
        Bp[col as usize] += 1;
    }

    let mut last: i32 = 0;
    for i in 0..((n_col+1) as usize) {
        temp = Bp[i];
        Bp[i] = last as i32;
        last = temp;
    }

    // transform function:

    let Bx = Bk.iter().map(|k| Ax[*k as usize]).collect();
    return (Bi, Bp, Bx);
}

#[allow(non_snake_case)]
pub fn csc_to_coo(
    Bi: &[i32],
    Bp: &[i32],
    Bx: &[Complex64],
) -> (Vec<i32>, Vec<i32>, Vec<Complex64>) {
    let Ai: Vec<i32> = Bi.iter().map(|i| *i).collect();
    let Bx: Vec<Complex64> = Bx.iter().map(|x| *x).collect();
    let mut Aj = Vec::new();

    let Bp_iter = Bp.iter();
    let mut Bp_iter_next = Bp.iter();
    Bp_iter_next.next();
    for (i, (b1, b2)) in Bp_iter.zip(Bp_iter_next).enumerate() {
        for _ in 0..(b2-b1){
            Aj.push(i as i32);
        }
    }
    return (Ai, Aj, Bx);
}
