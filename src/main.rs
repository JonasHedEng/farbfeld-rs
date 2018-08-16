#![feature(int_to_from_bytes)]
extern crate png;

use std::io;
use std::io::Write;

fn write_header(handle: &mut io::StdoutLock, width: u32, height: u32) -> io::Result<()> {
    let magic: &[u8] = b"farbfeld";

    handle.write_all(magic)?;
    handle.write_all(&width.to_be().to_bytes())?;
    handle.write_all(&height.to_be().to_bytes())?;
    handle.flush()?;

    Ok(())
}


fn main() {
    let stdin = io::stdin();
    let decoder = png::Decoder::new(stdin.lock());
    let (info, mut reader) = decoder.read_info().unwrap();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    write_header(&mut handle, info.width as u32, info.height as u32).unwrap();

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

    while let Some(row) = reader.next_row().unwrap() {
        row.chunks(bit_depth_bytes * color_type_bytes)
            .map(|p| {
                match p.len() {
                    3 => [p[0], p[0], p[1], p[1], p[2], p[2], 0xFF, 0xFF],
                    4 => [p[0], p[0], p[1], p[1], p[2], p[2], p[3], p[3]],
                    6 => [p[0], p[1], p[2], p[3], p[4], p[5], 0xFF, 0xFF],
                    8 => [p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]],
                    _ => panic!("Error reading bytes")
                }
            }).for_each(|p: [u8; 8]| handle.write_all(&p).unwrap());
    }

    handle.flush().unwrap();

}
