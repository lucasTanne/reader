use byteorder::{BigEndian, ByteOrder};

// See https://www.w3.org/TR/REC-png.pdf

// See https://iter.ca/post/png/

// SIGNATURE represents the signature of a PNG file
pub const SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

// CHUNK_TYPE_IHDR represents the signature of an IHDR chunk type
const CHUNK_TYPE_IHDR: [u8; 4] = [0x49, 0x48, 0x44, 0x52];

// CHUNK_TYPE_IDAT represents the signature of an IDAT chunk type
const CHUNK_TYPE_IDAT: [u8; 4] = [0x49, 0x48, 0x52, 0x4d];

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

// SIGNATURE_LENGTH represents the PNG signature length
const SIGNATURE_LENGTH: usize = 8;

// CHUNK_HEADER_LENGTH represents the header chunk length
const CHUNK_HEADER_LENGTH: usize = 25;

// CHUNK_GAMA_LENGTH represents the header gama length
const CHUNK_GAMA_LENGTH: usize = 16;

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

/// new returns a new instance of a PNG struct
/// 
/// Validate the header and clone the given buffer
pub fn new(buf: &[u8]) -> Png {
    let header: Header = decode_header(&buf[SIGNATURE_LENGTH..]);
    println!("\n{}", header.to_string());

    let gama: Gama = decode_chunk_gama(&buf[SIGNATURE_LENGTH+CHUNK_HEADER_LENGTH..]);
    println!("\n{}", gama.to_string());

    // TODO rework in loop for read to be automatic
    let (color_space_information, read_csi) = decode_chunk_color_space_information(&buf[SIGNATURE_LENGTH+CHUNK_HEADER_LENGTH+CHUNK_GAMA_LENGTH..]);
    println!("\n{}", color_space_information.to_string());

    let (physical_dimensions, read_pd) = decode_chunk_physical_dimensions(&buf[SIGNATURE_LENGTH+CHUNK_HEADER_LENGTH+CHUNK_GAMA_LENGTH+read_csi..]);
    println!("\n{}", physical_dimensions.to_string());

    // let (last_modification_date, read_lmd) = decode_chunk_last_modification_date(&buf[SIGNATURE_LENGTH+CHUNK_HEADER_LENGTH+CHUNK_GAMA_LENGTH+read_csi+read_pd..]);
    // println!("\n{}", last_modification_date.to_string());

    // let background_color: BackgroundColor = decode_chunk_background_color(&buf[SIGNATURE_LENGTH+CHUNK_HEADER_LENGTH+CHUNK_GAMA_LENGTH+read_csi+read_pd+read_lmd..]);
    // println!("\n{}", background_color.to_string());

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
/// buf is an array of byte which the file signature and the header must be removed
fn decode_chunk_gama(buf: &[u8]) -> Gama {
    // TODO use length to valid the gama chunk
    // let length: u32 = BigEndian::read_u32(&buf[..4]);

    let mut typ: [u8; 4] = [0; 4];
    typ.copy_from_slice(&buf[4..8]);
    if typ.cmp(&CHUNK_TYPE_GAMA).is_ne() {
        panic!("Chunk must be of type Gama")
    }

    let mut crc: [u8; 4] = [0; 4];
    crc.copy_from_slice(&buf[12..16]);

    return Gama {
        value: BigEndian::read_u32(&buf[8..12]),
        crc: crc
    }
}

/// decode_chunk_color_space_information decodes the color space information chunk of the given content
/// 
/// buf is an array of byte which the file signature, the header and gama chunk must be removed
fn decode_chunk_color_space_information(buf: &[u8]) -> (ColorSpaceInformation, usize) {
    let length: u32 = BigEndian::read_u32(&buf[..4]);

    let mut typ: [u8; 4] = [0; 4];
    typ.copy_from_slice(&buf[4..8]);
    if typ.cmp(&CHUNK_TYPE_COLOR_SPACE_INFORMATION).is_ne() {
        panic!("Chunk must be of type Color space information")
    }

    let u_length: usize = usize::try_from(length).unwrap();
    let mut crc: [u8; 4] = [0; 4];
    crc.copy_from_slice(&buf[8+u_length..8+u_length+4]);

    let read: usize = u_length + 4 + 4 + 4;

    return (
        ColorSpaceInformation {
            content: buf[8..8+u_length].to_vec(),
            crc: crc
        },
        read
    );
}

/// decode_chunk_physical_dimensions decodes the physical dimensions chunk of the given content
/// 
/// buf is an array of byte which the file signature, the header, gama and color space information chunk must be removed
fn decode_chunk_physical_dimensions(buf: &[u8]) -> (PhysicalDimensions, usize) {
    let length: u32 = BigEndian::read_u32(&buf[..4]);

    let mut typ: [u8; 4] = [0; 4];
    typ.copy_from_slice(&buf[4..8]);
    if typ.cmp(&CHUNK_TYPE_PHYSICAL_DIMENSIONS).is_ne() {
        panic!("Chunk must be of type pysical dimensions")
    }

    let u_length: usize = usize::try_from(length).unwrap();
    let mut crc: [u8; 4] = [0; 4];
    crc.copy_from_slice(&buf[8+u_length..8+u_length+4]);

    return (
        PhysicalDimensions {
            value: BigEndian::read_u32(&buf[8..8+u_length]),
            crc: crc
        },
        u_length
    );
}

/// decode_chunk_last_modification_date decodes the last modification date chunk of the given content
/// 
/// buf is an array of byte which the file signature, the header, gama, color space information and physical dimensions chunk must be removed
fn decode_chunk_last_modification_date(buf: &[u8]) -> (LastModificationDate, usize) {
    let length: u32 = BigEndian::read_u32(&buf[..4]);

    let mut typ: [u8; 4] = [0; 4];
    typ.copy_from_slice(&buf[4..8]);
    if typ.cmp(&CHUNK_TYPE_LAST_MODIFICATION_DATE).is_ne() {
        panic!("Chunk must be of type last modification date, got {:?}", typ)
    }

    let u_length: usize = usize::try_from(length).unwrap();
    let mut crc: [u8; 4] = [0; 4];
    crc.copy_from_slice(&buf[8+u_length..8+u_length+4]);

    return (
        LastModificationDate {
            value: buf[8..8+u_length].to_vec(),
            crc: crc
        },
        u_length
    );
}

/// decode_chunk_background_color decodes the background color chunk of the given content
/// 
/// buf is an array of byte which the file signature, the header, gama, color space information, physical dimensions and last modification date chunk must be removed
fn decode_chunk_background_color(buf: &[u8]) -> BackgroundColor {
    let length: u32 = BigEndian::read_u32(&buf[..4]);

    let mut typ: [u8; 4] = [0; 4];
    typ.copy_from_slice(&buf[4..8]);
    if typ.cmp(&CHUNK_TYPE_BACKGROUND_COLOR).is_ne() {
        panic!("Chunk must be of type background color, got {:?}", typ)
    }

    let u_length: usize = usize::try_from(length).unwrap();
    let mut crc: [u8; 4] = [0; 4];
    crc.copy_from_slice(&buf[8+u_length..8+u_length+4]);

    return BackgroundColor {
        value: buf[8..8+u_length].to_vec(),
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
        // chunk_length_size represents the chunk's data length (4 byte)
        let chunk_length_size: usize = 4;

        // chunk_type_size represents the chunk's type length (4 byte)
        let chunk_type_size: usize = 4;

        let content: &[u8] = self.content.as_slice();
        let content_length: usize = self.content.len();
        println!("CONTENT LENGTH: {}", content_length);
        let mut i: usize = 0;
        loop {
            let copied_content: &[u8] = &content[i..];
            let chunk_length: u32 = BigEndian::read_u32(&copied_content[0..chunk_length_size]);
            let chunk_type: &[u8] = &copied_content[chunk_length_size..(chunk_length_size+chunk_type_size)];

            if chunk_type.cmp(&CHUNK_TYPE_IDAT).is_eq() {
                println!("This an IDAT chunk");
            } else {
                println!("THIS IN NOT AN IDAT, skipping...");
            }    

            let u_chunk_length: usize = usize::try_from(chunk_length).unwrap();
            i = i + chunk_length_size+chunk_type_size+u_chunk_length;

            if i >= content_length {
                println!("Finished");
                return
            }
        }
    }
}

