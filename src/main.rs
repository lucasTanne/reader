use std::{fs::File, io::{BufRead, BufReader}};

fn main() {
    println!("Starting image reader...");

    let file_path = "./cat.jpg";

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => panic!("Could not open the file: {}, err: {}", file_path, err)
    };

    // READ as string

    // let mut content = String::new();
    // let content_size = match file.read_to_string(&mut content) {
    //     Ok(size) => size,
    //     Err(err) => panic!("Could not read file content, got error: {}", err)
    // };

    // println!("File content size: {}", content_size);

    // println!("File content: {}", content);

    // READ as byte buf

    let mut reader = BufReader::new(file);
    let content = match reader.fill_buf() {
        Ok(content) => content,
        Err(err) => panic!("Could not fill buffer, got error {}", err)
    };

    println!("File content size: {}", content.len());

    for b in content {
        print!("{} ", b)
    }

}
