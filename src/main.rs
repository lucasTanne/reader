use std::{fs::File, io::Read};

mod png;
mod gui;

fn is_png_file(file_header: &[u8]) -> bool {
    if file_header.cmp(&png::SIGNATURE).is_eq() {
        return true
    }

    return false
}

fn main() {
    println!("Starting image reader...");

    let file_path: &str = &"./s.png";

    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(err) => panic!("Could not open the file: {}, err: {}", &file_path, err)
    };

    let mut content = Vec::new();
    file.read_to_end(&mut content);

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

    let file_s: png::Png = png::new(content.as_slice());
    file_s.read();

    // gui::run();
}
