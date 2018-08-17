#![feature(int_to_from_bytes)]
extern crate png;
extern crate time;

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

    // READ
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).expect("Could not read data");

    // WRITE
    let mut buffer = Vec::with_capacity(8 * info.buffer_size() / 3);

    match (info.bit_depth, info.color_type) {
        (png::BitDepth::Eight, png::ColorType::RGB)  =>
            buf.chunks(3)
            .map(|p| [p[0], p[0], p[1], p[1], p[2], p[2], 0xFF, 0xFF])
            .for_each(|p| buffer.extend(&p)),

        (png::BitDepth::Eight, png::ColorType::RGBA) =>
            buf.chunks(4)
            .map(|p| [p[0], p[0], p[1], p[1], p[2], p[2], p[3], p[3]])
            .for_each(|p| buffer.extend(&p)),

        (png::BitDepth::Sixteen, png::ColorType::RGB)  =>
            buf.chunks(6)
            .map(|p| [p[0], p[1], p[2], p[3], p[4], p[5], 0xFF, 0xFF])
            .for_each(|p| buffer.extend(&p)),

        (png::BitDepth::Sixteen, png::ColorType::RGBA) =>
            buf.chunks(8)
            .map(|p| [p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]])
            .for_each(|p| buffer.extend(&p)),

        (_, _) => panic!("Unsupported PNG type")
    }

    handle.write_all(&buffer).unwrap();
    handle.flush().unwrap();

}
