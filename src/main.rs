#![feature(int_to_from_bytes)]
extern crate png;
extern crate rayon;

use std::io;
use std::io::Write;

use rayon::prelude::*;

fn write_header(width: u32, height: u32) -> io::Result<()> {
    let magic: Vec<u8> = b"farbfeld".to_vec();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write(&magic)?;
    handle.write(&width.to_be().to_bytes())?;
    handle.write(&height.to_be().to_bytes())?;
    handle.flush()?;

    Ok(())
}


fn main() {
    let stdin = io::stdin();
    let decoder = png::Decoder::new(stdin.lock());
    let (info, mut reader) = decoder.read_info().unwrap();

    write_header(info.width as u32, info.height as u32).unwrap();

    let mut img: Vec<u8> = Vec::new();
    while let Some(row) = reader.next_row().unwrap() {
        // Row amount is equal to height
        img.extend(row);
    }

    let bit_depth_bytes = match info.bit_depth {
        png::BitDepth::Eight => 1,
        png::BitDepth::Sixteen => 2,
        _ => panic!("Invalid bit-depth")
    };

    let color_type_bytes = match info.color_type {
        png::ColorType::RGB => 3,
        png::ColorType::RGBA => 4,
        _ => panic!("Invalid color-type")
    };

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    img.par_chunks(bit_depth_bytes * color_type_bytes).map(|p| {
        match p.len() {
            3 => [p[0], p[0], p[1], p[1], p[2], p[2], 0xFF, 0xFF],
            4 => [p[0], p[0], p[1], p[1], p[2], p[2], p[3], p[3]],
            6 => [p[0], p[1], p[2], p[3], p[4], p[5], 0xFF, 0xFF],
            8 => [p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]],
            _ => panic!("Error reading bytes")
        }
    }).collect::<Vec<_>>().iter().for_each(|p: &[u8; 8]| handle.write_all(p).unwrap());

    handle.flush().unwrap();

}
