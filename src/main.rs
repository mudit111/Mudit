use std::fs;
mod write_data_to_file;
mod write_to_string;

fn main() {
    let str = write_to_string::write_to_string();
    write_data_to_file::write_to_file(str);
}


j