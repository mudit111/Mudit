use s3::Bucket;
use std::fs::File;
use std::io::Read;

pub fn upload_image(path: &str, file_name :&str, bucket :&Bucket){

    let mut file_content:Vec<u8> = Vec::new();
    let  mut file: File = File::open(path).expect("unable to read the data");
    file.read_to_end(&mut file_content).unwrap();
    bucket.put(file_name,file_content.as_slice(),"image/jpeg").expect("unable to upload file");
}