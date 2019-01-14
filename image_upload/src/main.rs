use s3::Bucket;
pub mod s3_connection;
pub mod text_upload;
fn main() {
    //println!("Hello, world!");
    let bucket: Bucket = s3_connection::connection();
    text_upload::upload_text("/home/knoldus/sample.text","sample.txt",&bucket);
}
