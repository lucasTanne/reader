use core::panic;
use std::io::Read;

use byteorder::{BigEndian, ByteOrder};
use flate2::bufread::ZlibDecoder;

// See https://www.w3.org/TR/REC-png.pdf

// See https://iter.ca/post/png/

// SIGNATURE represents the signature of a PNG file
pub const SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

// CHUNK_TYPE_IHDR represents the signature of an IHDR chunk type
const CHUNK_TYPE_IHDR: [u8; 4] = [0x49, 0x48, 0x44, 0x52];

// CHUNK_TYPE_GAMA represents the signature of a gAMA chunk type
const CHUNK_TYPE_GAMA: [u8; 4] = [0x67, 0x41, 0x4d, 0x41];

// CHUNK_TYPE_COLOR_SPACE_INFORMATION represents the signature of a color space information chunk type
const CHUNK_TYPE_COLOR_SPACE_INFORMATION: [u8; 4] = [0x63, 0x48, 0x52, 0x4d];

// CHUNK_TYPE_PHYSICAL_DIMENSIONS represents the signature of a pysical dimensisions chunk type
const CHUNK_TYPE_PHYSICAL_DIMENSIONS: [u8; 4] = [0x70, 0x48, 0x59, 0x73];

// CHUNK_TYPE_LAST_MODIFICATION_DATE represents the signature of a last modification date chunk type
const CHUNK_TYPE_LAST_MODIFICATION_DATE: [u8; 4] = [0x74, 0x49, 0x4d, 0x45];

// CHUNK_TYPE_BACKGROUND_COLOR represents the signature of a background color chunk type
const CHUNK_TYPE_BACKGROUND_COLOR: [u8; 4] = [0x62, 0x4b, 0x47, 0x44];

// CHUNK_TYPE_TEXT represents the signature of a text chunk type
const CHUNK_TYPE_TEXT: [u8; 4] = [0x74, 0x45, 0x58, 0x74];

// CHUNK_TYPE_IDAT represents the signature of an IDAT chunk type
const CHUNK_TYPE_IDAT: [u8; 4] = [0x49, 0x44, 0x41, 0x54];

// CHUNK_TYPE_END represents the signature of a end chunk type
const CHUNK_TYPE_END: [u8; 4] = [0x49, 0x45, 0x4E, 0x44];

// SIGNATURE_LENGTH represents the PNG signature length (8 bytes)
const SIGNATURE_LENGTH: usize = 8;

// CHUNK_LENGTH_SIZE represents the length of the chunk's length (4 bytes)
const CHUNK_LENGTH_SIZE: usize = 4;

// CHUNK_TYPE_SIZE represents the length of the chunk's type (4 byte)
const CHUNK_TYPE_SIZE: usize = 4;

// CHUNK_CRC_SIZE represents the length of the chunk's CRC (4 byte)
const CHUNK_CRC_SIZE: usize = 4;

// CHUNK_HEADER_LENGTH represents the header chunk length
const CHUNK_HEADER_LENGTH: usize = 25;

struct Header {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_space: u8,
    compression_method: u8,
    filter_method: u8,
    interlacing: u8,

    crc: [u8; 4]
}

impl ToString for Header {
    fn to_string(&self) -> String {
        return format!("## Header\nWidth: {}\nLength: {}\nBit depth: {}\nColor space: {}\nCompression method: {}\nFilter method: {}\nInterlacing: {}\nCRC: {:?}",
            self.width,
            self.height,
            self.bit_depth,
            self.color_space,
            self.compression_method,
            self.filter_method,
            self.interlacing,
            self.crc
        );
    }
}

struct Gama {
    value: u32,
    crc: [u8; 4]
}

impl ToString for Gama {
    fn to_string(&self) -> String {
        return format!("## Gama\nValue: {}\nCRC: {:?}", self.value, self.crc);
    }
}

struct ColorSpaceInformation {
    content: Vec<u8>, // TODO handle content's values
    crc: [u8; 4]
}

impl ToString for ColorSpaceInformation {
    fn to_string(&self) -> String {
        return format!("## Color space information\nContent: TODO\nCRC: {:?}", self.crc);
    }
}

struct PhysicalDimensions {
    value: u32,
    crc: [u8; 4]
}

impl ToString for PhysicalDimensions {
    fn to_string(&self) -> String {
        return format!("## Phisical dimensions\nValue: {}\nCRC: {:?}", self.value, self.crc);
    }
}

struct LastModificationDate {
    value: Vec<u8>, // TODO handle UTC date
    crc: [u8; 4]
}

impl ToString for LastModificationDate {
    fn to_string(&self) -> String {
        return format!("## Last modification date\nValue: {:?}\nCRC: {:?}", self.value, self.crc);
    }
}

struct BackgroundColor {
    value: Vec<u8>, // TODO handle color
    crc: [u8; 4]
}

impl ToString for BackgroundColor {
    fn to_string(&self) -> String {
        return format!("## Background color\nValue: {:?}\nCRC: {:?}", self.value, self.crc);
    }
}

struct ImageData {
    content: Vec<u8>,
    crc: [u8; 4]
}

impl ToString for ImageData {
    fn to_string(&self) -> String {
        return format!("## Image data\nCRC: {:?}", self.crc);
    }
}

struct Text {
    content: Vec<u8>,
    crc: [u8; 4]
}

impl ToString for Text {
    fn to_string(&self) -> String {
        return format!("## Text\nContent: {}\nCRC: {:?}", String::from_utf8(self.content.clone()).unwrap(), self.crc);
    }
}

struct End {
    crc: [u8; 4]
}

impl ToString for End {
    fn to_string(&self) -> String {
        return format!("## End\nCRC: {:?}", self.crc);
    }
}

/// new returns a new instance of a PNG struct
/// 
/// Validate the header and clone the given buffer
pub fn new(buf: &[u8]) -> Png {
    let header: Header = decode_header(&buf[SIGNATURE_LENGTH..]);
    println!("\n{}", header.to_string());

    return Png {
        header,
        content: buf[SIGNATURE_LENGTH+CHUNK_HEADER_LENGTH..].to_vec()
    }
}

/// decode_header decodes the header of a given PNG file
/// 
/// buf is an array of byte which the file's signature must be removed (signature should be the 8 first byte)
fn decode_header(buf: &[u8]) -> Header{
    // Panic if the header's data length isn't 13 byte, cannot handle other
    let length: u32 = BigEndian::read_u32(&buf[..4]);
    if length != 13 {
        panic!("Header must be 25 bytes, got: {}", length)
    }

    let mut typ: [u8; 4] = [0; 4];
    typ.copy_from_slice(&buf[4..8]);
    if typ.cmp(&CHUNK_TYPE_IHDR).is_ne() {
        panic!("Fist chunk must be of type IHDR")
    }

    let u_length: usize = usize::try_from(length).unwrap();
    let mut crc: [u8; 4] = [0; 4];
    crc.copy_from_slice(&buf[8+u_length..(8+u_length)+4]);

    return Header {
        width: BigEndian::read_u32(&buf[8..12]),
        height: BigEndian::read_u32(&buf[12..16]),
        bit_depth: buf[16],
        color_space: buf[17],
        compression_method: buf[18],
        filter_method: buf[19],
        interlacing: buf[20],
        crc: crc
    }
}

/// decode_chunk_gama decodes the gama chunk of the given content
/// 
/// buf is an array of byte and data_length is the number of byte to read from buf
fn decode_chunk_gama(buf: &[u8], data_length: usize) -> Gama {
    let mut crc: [u8; 4] = [0; 4];
    crc.copy_from_slice(&buf[data_length..data_length+CHUNK_CRC_SIZE]);

    return Gama {
        value: BigEndian::read_u32(&buf[..data_length]),
        crc: crc
    }
}

/// decode_chunk_color_space_information decodes the color space information chunk of the given content
/// 
/// buf is an array of byte and data_length is the number of byte to read from buf
fn decode_chunk_color_space_information(buf: &[u8], data_length: usize) -> ColorSpaceInformation {
    let mut crc: [u8; 4] = [0; 4];
    crc.copy_from_slice(&buf[data_length..data_length+CHUNK_CRC_SIZE]);

    return ColorSpaceInformation {
        content: buf[..data_length].to_vec(),
        crc: crc
    }
}

/// decode_chunk_physical_dimensions decodes the physical dimensions chunk of the given content
/// 
/// buf is an array of byte and data_length is the number of byte to read from buf
fn decode_chunk_physical_dimensions(buf: &[u8], data_length: usize) -> PhysicalDimensions {
    let mut crc: [u8; 4] = [0; 4];
    crc.copy_from_slice(&buf[data_length..data_length+CHUNK_CRC_SIZE]);

    return PhysicalDimensions {
        value: BigEndian::read_u32(&buf[..data_length]),
        crc: crc
    }
}

/// decode_chunk_last_modification_date decodes the last modification date chunk of the given content
/// 
/// buf is an array of byte and data_length is the number of byte to read from buf
fn decode_chunk_last_modification_date(buf: &[u8], data_length: usize) -> LastModificationDate {
    let mut crc: [u8; 4] = [0; 4];
    crc.copy_from_slice(&buf[data_length..data_length+CHUNK_CRC_SIZE]);

    return LastModificationDate {
            value: buf[..data_length].to_vec(),
            crc: crc
        }
}

/// decode_chunk_background_color decodes the background color chunk of the given content
/// 
/// buf is an array of byte and data_length is the number of byte to read from buf
fn decode_chunk_background_color(buf: &[u8], data_length: usize) -> BackgroundColor {
    let mut crc: [u8; 4] = [0; 4];
    crc.copy_from_slice(&buf[data_length..data_length+CHUNK_CRC_SIZE]);

    return BackgroundColor {
        value: buf[..data_length].to_vec(),
        crc: crc
    }
}

/// decode_chunk_text decodes the text chunk of the given content
/// 
/// buf is an array of byte and data_length is the number of byte to read from buf
fn decode_chunk_text(buf: &[u8], data_length: usize) -> Text {
    let mut crc: [u8; 4] = [0; 4];
    crc.copy_from_slice(&buf[data_length..data_length+CHUNK_CRC_SIZE]);

    return Text {
        content: buf[..data_length].to_vec(),
        crc: crc
    }
}

/// decode_chunk_end decodes the end chunk of the given content
/// 
/// buf is an array of byte and data_length is the number of byte to read from buf
fn decode_chunk_end(buf: &[u8], data_length: usize) -> End {
    let mut crc: [u8; 4] = [0; 4];
    crc.copy_from_slice(&buf[data_length..data_length+CHUNK_CRC_SIZE]);

    return End {
        crc: crc
    }
}

/// decode_chunk_image_data decodes the image data chunk of the given content
/// 
/// buf is an array of byte and data_length is the number of byte to read from buf
///
/// buf must be a single Zlib compressed stream
fn decode_chunk_image_data(buf: &[u8], data_length: usize) -> ImageData {
    println!("CMF: {:#x}", buf[0]);
    println!("FLG: {:#x}", buf[1]);
    if buf.starts_with(&[0x78, 0xDA]) { // TODO func
        println!("ZLIB");
    }

    let mut decoder = ZlibDecoder::new(buf);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed);
    let decompressed_len: usize = decompressed.len();
    println!("Decompressed : {:?}", decompressed_len);

    let mut crc: [u8; 4] = [0; 4];
    crc.copy_from_slice(&decompressed[decompressed_len-CHUNK_CRC_SIZE..decompressed_len]);

    return ImageData { 
        content: decompressed,
        crc: crc
    }
}

pub struct Png {
    header: Header,
    content: Vec<u8>
}

impl Png {
    /// read reads the given file's content
    pub fn read(&self) {
        println!("\nStart reading...");

        let content: &[u8] = self.content.as_slice();
        let u_content_length: usize = self.content.len();
        println!("CONTENT LENGTH: {}", u_content_length);

        let mut u_read: usize = 0;
        loop {
            let chunk_length: u32 = BigEndian::read_u32(&content[u_read..u_read+CHUNK_LENGTH_SIZE]);
            let u_chunk_length: usize = usize::try_from(chunk_length).unwrap();
            println!("\nChunk length: {u_chunk_length}");

            let chunk_type: &[u8] = &content[u_read+CHUNK_LENGTH_SIZE..u_read+CHUNK_LENGTH_SIZE+CHUNK_TYPE_SIZE];

            let u_chunk_header_readed: usize = CHUNK_LENGTH_SIZE+CHUNK_TYPE_SIZE;
            let chunk_content: &[u8] = &content[u_read+u_chunk_header_readed..];

            if chunk_type.cmp(&CHUNK_TYPE_GAMA).is_eq() {
                let chunk: Gama = decode_chunk_gama(chunk_content, u_chunk_length);
                println!("{}", chunk.to_string());
            } else if chunk_type.cmp(&CHUNK_TYPE_COLOR_SPACE_INFORMATION).is_eq() {
                let chunk: ColorSpaceInformation = decode_chunk_color_space_information(chunk_content, u_chunk_length);
                println!("{}", chunk.to_string());
            } else if chunk_type.cmp(&CHUNK_TYPE_PHYSICAL_DIMENSIONS).is_eq() {
                let chunk: PhysicalDimensions = decode_chunk_physical_dimensions(chunk_content, u_chunk_length);
                println!("{}", chunk.to_string());
            } else if chunk_type.cmp(&CHUNK_TYPE_LAST_MODIFICATION_DATE).is_eq() {
                let chunk: LastModificationDate = decode_chunk_last_modification_date(chunk_content, u_chunk_length);
                println!("{}", chunk.to_string());
            } else if chunk_type.cmp(&CHUNK_TYPE_BACKGROUND_COLOR).is_eq() {
                let chunk: BackgroundColor = decode_chunk_background_color(chunk_content, u_chunk_length);
                println!("{}", chunk.to_string());
            } else if chunk_type.cmp(&CHUNK_TYPE_TEXT).is_eq() {
                let chunk: Text = decode_chunk_text(chunk_content, u_chunk_length);
                println!("{}", chunk.to_string());
            } else if chunk_type.cmp(&CHUNK_TYPE_IDAT).is_eq() {
                println!("ImageDATA");
                
                let chunk: ImageData = decode_chunk_image_data(chunk_content, u_chunk_length);
                println!("\n{}", chunk.to_string());
            } else if chunk_type.cmp(&CHUNK_TYPE_END).is_eq() {
                let chunk: End = decode_chunk_end(chunk_content, u_chunk_length);
                println!("{}", chunk.to_string());
                break;
            } else {
                println!("Chunk type is not recognized");
                for b in chunk_type {
                    print!("{:#x} ", b)
                }
                println!("");
                let chunk_type_str = String::from_utf8(chunk_type.to_vec()).unwrap();
                println!("TYPE: {chunk_type_str}");
                panic!("Abording");
            }

            u_read += u_chunk_header_readed+u_chunk_length+CHUNK_CRC_SIZE;
            if u_read == u_content_length {
                break;
            }
        }

        println!("\nEND");
    }
}

