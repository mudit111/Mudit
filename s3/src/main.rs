use s3::Bucket;

pub mod s3_connection;
pub mod text_upload;
pub mod image_upload;

fn main() {
   let bucket: Bucket = s3_connection::connection();
   //to uplaod text file
   text_upload::upload_text("/home/knoldus/file1.txt", "file1.txt", &bucket);
   //to upload image file
   image_upload::upload_image("/home/knoldus/rust.png", "rust.png", &bucket);

}


/*  here we store the bucket name
    let name: &str = "hawktestbucket";
    //here we select the region correspondent to our select region while creatiion of the s3 bucket
    // let region:Region = "ap-south-1".parse().expect("unable to validate region");
    let region: Region = Region::ApSouth1;
    //it reads and loads the credentials from the environment where access key and secret key id provided by us and
    //we initialize the credentials directly with the new function
    let credentials: Credentials = Credentials::new("AKIAIHW6UQ53LA5LJOQA", "TxwID2zpkpWBnXSYIfbVUKgNAntHWYVNhfCpPpi5", None);
    //here we use the new fucntion of the bucket and create the instance of the bucket
    let bucket: Bucket = Bucket::new(name, region, credentials);//bucket instantiation
    */



    //---------text file upload starts-------------
    /* let data:String = fs::read_to_string("/home/knoldus/file2.txt").expect("unable to read the data");
    let content= data.as_bytes();
    bucket.put("file2.txt",content,"test/plain").expect("unable to upload file");
    */
    //---------text file upload ends---------------




    //--------------image upload starts--------------
    /*let mut file_content = Vec::new();
    let  mut file = File::open("/home/knoldus/image.jpg").expect("unable to read the data");
    file.read_to_end(&mut file_content).unwrap();
    bucket.put("image.jpg",file_content.as_slice(),"image/jpeg").expect("unable to upload file");*/
    //-------------image upload ends--------------



