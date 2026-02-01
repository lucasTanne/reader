use std::{fs::File, io::{BufRead, BufReader}, str::FromStr};
use byteorder::{ByteOrder, BigEndian};

// See https://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

const PNG_CHUNK_TYPE_IHDR: [u8; 4] = [0x49, 0x48, 0x44, 0x52];

fn is_png_file(file_header: &[u8]) -> bool {
    if file_header.cmp(&PNG_SIGNATURE).is_eq() {
        return true
    }

    return false
}

struct FilePNG {
    file_path: String,
    content: Vec<u8>
}

impl FilePNG {
    fn decode_first_chunk(&self) -> Chunk {
        // Retrieve chunk "length" of data chunk (4 byte)
        // its value must not exceed 2^31 bytes
        // = 13
        // let chunk_length = &self.content[8..12];
        // println!("Chunk length:");
        // for b in chunk_length {
        //     print!("{:#x} ", b)
        // }
        // println!("");
        let chunk_length = BigEndian::read_u32(&self.content[0..4]);

        // Retrieve the chunk "type" (4 byte)
        // let chunk_type = &self.content[12..16];
        // println!("Chunk type:");
        // for b in chunk_type {
        //     print!("{:#x} ", b)
        // }
        // Read chunk type as str
        let chunk_type_str = match str::from_utf8(&self.content[4..8]) {
            Err(err) => panic!("Could not convert chunk type into str, got err: {}", err),
            Ok(t) => t
        };
        // println!("\ntype: {}", chunk_type_str);
        // println!("");

        // Retrieve the chunk "data" (16 + chunk length)
        // convert chunk len into usize => (16 + l).try_into().unwrap()
        let data_last_index = (8 + chunk_length).try_into().unwrap();
        let chunk_data = &self.content[16..data_last_index];
        // println!("Chunk data:");
        // for b in chunk_data {
        //     print!("{:#x} ", b)
        // }
        // println!("");

        // Retrieve the CRC (4 byte)
        let crc_last_index = (data_last_index + 4).try_into().unwrap();
        let chunk_crc = &self.content[data_last_index..crc_last_index];
        // println!("Chunk crc:");
        // for b in chunk_crc {
        //     print!("{:#x} ", b)
        // }

        let chunk: Chunk = Chunk {
            length: chunk_length,
            type_str: String::from_str(chunk_type_str).expect("Could not read chunk type as String"),
            data: Vec::from(chunk_data),
            crc: Vec::from(chunk_crc)
        };

        return chunk;
    }
}

#[derive(Debug)]
struct Chunk {
    length: u32,
    type_str: String,
    data: Vec<u8>,
    crc: Vec<u8>
}

fn main() {
    println!("Starting image reader...");

    let file_path = String::from("./cat.png");

    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => panic!("Could not open the file: {}, err: {}", &file_path, err)
    };

    let mut reader = BufReader::new(file);
    let content = match reader.fill_buf() {
        Ok(content) => content,
        Err(err) => panic!("Could not fill buffer, got error {}", err)
    };

    println!("File content size: {}", content.len());

    // Retrieve signature
    let file_signature = &content[..8];
    println!("File signature:");
    for b in file_signature {
        print!("{:#x} ", b)
    }
    println!("");

    // Compare signature to PNG
    if !is_png_file(file_signature) {
        panic!("This is not a PNG file");
    }

    let f: FilePNG = FilePNG {
        file_path: file_path,
        content: Vec::from(&content[8..]) // content without signature
    };

    let chunk = f.decode_first_chunk();
    println!("First chunk: {:#?}", chunk);
}
