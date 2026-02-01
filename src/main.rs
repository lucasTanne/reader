use std::{fs::File, io::{BufRead, BufReader}};

// See https://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

fn main() {
    println!("Starting image reader...");

    let file_path = "./cat.png";

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => panic!("Could not open the file: {}, err: {}", file_path, err)
    };

    let mut reader = BufReader::new(file);
    let content = match reader.fill_buf() {
        Ok(content) => content,
        Err(err) => panic!("Could not fill buffer, got error {}", err)
    };

    println!("File content size: {}", content.len());

    // // Print the byte stream
    // for b in content {
    //     print!("{} ", b)
    // }

    // Compare PNG signature
    let file_signature = &content[..8];
    if !file_signature.cmp(&PNG_SIGNATURE).is_eq() {
        panic!("This is not a PNG file, signature doesn't match");
    }

}
