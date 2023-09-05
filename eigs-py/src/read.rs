use num_complex::Complex;
use std::fs::{metadata, File};
use std::io::Read;

pub fn parse_f32_vec(bts: &Vec<u8>) -> Vec<f32> {
    let chunks = chunks(&bts, 4);
    return chunks
        .into_iter()
        .map(|a| f32::from_le_bytes(chunk4(a)))
        .collect();
}

pub fn parse_f64_vec(bts: &Vec<u8>) -> Vec<f64> {
    let chunks = chunks(&bts, 8);
    return chunks
        .into_iter()
        .map(|a| f64::from_le_bytes(chunk8(a)))
        .collect();
}

pub fn parse_c32_vec(bts: &Vec<u8>) -> Vec<Complex<f32>> {
    let chunks_f32 = chunks(&bts, 4);
    let vecf32: Vec<f32> = chunks_f32
        .into_iter()
        .map(|a| f32::from_le_bytes(chunk4(a)))
        .collect();
    let chunks_c32 = chunks(&vecf32, 2);
    return chunks_c32
        .into_iter()
        .map(|v| Complex { re: v[0], im: v[1] })
        .collect();
}

pub fn parse_c64_vec(bts: &Vec<u8>) -> Vec<Complex<f64>> {
    let chunks_f64 = chunks(&bts, 8);
    let vecf64: Vec<f64> = chunks_f64
        .into_iter()
        .map(|a| f64::from_le_bytes(chunk8(a)))
        .collect();
    let chunks_c64 = chunks(&vecf64, 2);
    return chunks_c64
        .into_iter()
        .map(|v| Complex { re: v[0], im: v[1] })
        .collect();
}

pub fn parse_i32_vec(bts: &Vec<u8>) -> Vec<i32> {
    let chunks = chunks(&bts, 4);
    return chunks
        .into_iter()
        .map(|a| i32::from_le_bytes(chunk4(a)))
        .collect();
}

pub fn read_u8(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).expect(&format!("no file '{filename}' found."));
    let meta = metadata(filename).expect(&format!("unable to read '{filename}' metadata."));
    let mut buffer = vec![0; meta.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    return buffer;
}

fn chunks<T>(vec: &Vec<T>, n: usize) -> impl Iterator<Item = &[T]> {
    return (0..vec.len()).step_by(n).map(move |i| &vec[i..i + n]);
}

fn chunk4(v: &[u8]) -> [u8; 4] {
    let mut a = [0u8; 4];
    for (i, p) in a.iter_mut().enumerate() {
        *p = v[i];
    }
    return a;
}

fn chunk8(v: &[u8]) -> [u8; 8] {
    let mut a = [0u8; 8];
    for (i, p) in a.iter_mut().enumerate() {
        *p = v[i];
    }
    return a;
}
