use png::{Encoder, Decoder, ColorType, BitDepth};

extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn encode_png(data: &Vec<u8>, width: u32) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    let height = data.len() as u32 / (4 * width);
    {
        let mut encoder = Encoder::new(&mut output, width, height);
        encoder.set_color(ColorType::RGBA);
        encoder.set_depth(BitDepth::Eight);
        let mut writer = match encoder.write_header() {
            Ok(writer) => writer,
            Err(error) => return Err(error.to_string())
        };
        match writer.write_image_data(&data) {
            Err(error) => return Err(error.to_string()),
            _ => ()
        };
    }
    Ok(output)
}

pub struct InternalImageData {
    pub buffer: Vec<u8>,
    pub width: u32,
    pub height: u32
}

pub fn decode_png(compressed: &[u8]) -> Result<InternalImageData, String> {
    let decoder = Decoder::new(compressed);
    let (info, mut reader) = match decoder.read_info() {
        Ok(info_reader) => info_reader,
        Err(error) => return Err(error.to_string())
    };
    if info.color_type != ColorType::RGBA {
        return Err("PNG decoding only supports RGBA at the moment".into());
    };
    if info.bit_depth != BitDepth::Eight {
        return Err("PNG decoding only supports 8 bits per component at the moment".into());
    };

    let mut buffer = Vec::<u8>::new();
    buffer.resize(info.buffer_size(), 0);
    reader.next_frame(&mut buffer).map_err(|e| format!("Decoding PNG failed: {e}"))?;
    Ok(InternalImageData{ buffer, width: info.width, height: info.height })
}

fn main() {
    let mut counter = 0;
    loop {
        let width = 313;
        let height = 247;
        let buffer: Vec<u8> = (0..width*height)
            .flat_map(|_| [rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 255])
            .collect();
        assert_eq!(buffer.len(), width * height * 4);
        let compressed = encode_png(&buffer, width as u32).unwrap();
        let decoded = decode_png(&compressed).unwrap();
        assert_eq!(buffer, decoded.buffer);
        assert_eq!(width as u32, decoded.width);
        assert_eq!(height as u32, decoded.height);
        counter += 1;
        println!("Done {counter}");
    }
}
