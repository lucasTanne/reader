use std::{fs::File, io::{BufRead, BufReader}};

mod png;

fn is_png_file(file_header: &[u8]) -> bool {
    if file_header.cmp(&png::SIGNATURE).is_eq() {
        return true
    }

    return false
}

fn main() {
    println!("Starting image reader...");

    let file_path: &str = &"./s.png";

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

    let file: png::Png = png::new(content);
    file.read()
}
