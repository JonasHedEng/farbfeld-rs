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

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let decoder = png::Decoder::new(stdin.lock());
    let (info, mut reader) = decoder.read_info()?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    write_header(&mut handle, info.width as u32, info.height as u32).unwrap();

    // READ
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf)?;

    // Create out buffer
    let mut buffer = match (info.bit_depth, info.color_type) {
        (png::BitDepth::Eight, png::ColorType::RGB) =>
            vec![0u8; 8 * info.buffer_size() / 3],

        (png::BitDepth::Eight, png::ColorType::RGBA) =>
            vec![0u8; 8 * info.buffer_size() / 4],

        (png::BitDepth::Sixteen, png::ColorType::RGB) =>
            vec![0u8; 8 * info.buffer_size() / 6],

        (png::BitDepth::Sixteen, png::ColorType::RGBA) =>
            vec![0u8; 8 * info.buffer_size() / 8],

        (_, _) => panic!("Unsupported PNG type")
    };

    // Fill out buffer
    match (info.bit_depth, info.color_type) {
        (png::BitDepth::Eight, png::ColorType::RGB)  => {
            let size = 3;

            for (idx, p) in buf.chunks(size).enumerate() {
                let i = idx*size;
                buffer[i+0] = p[0];
                buffer[i+1] = p[0];
                buffer[i+2] = p[1];
                buffer[i+3] = p[1];
                buffer[i+4] = p[2];
                buffer[i+5] = p[2];
                buffer[i+6] = 0xFF;
                buffer[i+7] = 0xFF;
            }
        },

        (png::BitDepth::Eight, png::ColorType::RGBA) => {
            let size = 4;

            for (idx, p) in buf.chunks(size).enumerate() {
                let i = idx*size;
                buffer[i+0] = p[0];
                buffer[i+1] = p[0];
                buffer[i+2] = p[1];
                buffer[i+3] = p[1];
                buffer[i+4] = p[2];
                buffer[i+5] = p[2];
                buffer[i+6] = p[3];
                buffer[i+7] = p[3];
            }
        },

        (png::BitDepth::Sixteen, png::ColorType::RGB)  => {
            let size = 6;

            for (idx, p) in buf.chunks(size).enumerate() {
                let i = idx*size;
                buffer[i+0] = p[0];
                buffer[i+1] = p[1];
                buffer[i+2] = p[2];
                buffer[i+3] = p[3];
                buffer[i+4] = p[4];
                buffer[i+5] = p[5];
                buffer[i+6] = 0xFF;
                buffer[i+7] = 0xFF;
            }
        },

        (png::BitDepth::Sixteen, png::ColorType::RGBA) => {
            let size = 8;

            for (idx, p) in buf.chunks(size).enumerate() {
                let i = idx*size;
                buffer[i+0] = p[0];
                buffer[i+1] = p[1];
                buffer[i+2] = p[2];
                buffer[i+3] = p[3];
                buffer[i+4] = p[4];
                buffer[i+5] = p[5];
                buffer[i+6] = p[6];
                buffer[i+7] = p[7];
            }
        },

        (_, _) => panic!("Unsupported PNG type")
    }

    // Write
    handle.write_all(&buffer)?;
    handle.flush()?;

    Ok(())
}
