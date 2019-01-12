use s3::Bucket;
use std::fs;

pub fn upload_text(path:&str, file_name :&str, bucket :&Bucket){

    let data:String = fs::read_to_string(path).expect("unable to read the data");
    let content:&[u8]= data.as_bytes();
    bucket.put(file_name,content,"test/plain").expect("unable to upload file");
}