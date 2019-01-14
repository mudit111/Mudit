use s3::region::Region;
use s3::credentials::Credentials;
use s3::bucket::Bucket;
use std::fs::File;
use std::io::Read;

fn main() {
    
    	let bucket: Bucket = connection();
	let mut file_content:Vec<u8> = Vec::new();
    	let  mut file: File = File::open(path).expect("incorrect data");
 	file.read_to_end(&mut file_content).unwrap();
	image_upload("/home/knoldus/hello.png", "hello.png", &bucket);
	bucket.put("hello.png",file_content.as_slice(),"image/png");
}

pub fn connection() ->Bucket{

    
    	let bucket_name = "mudit";
	let access_key = String::from("AKIAIINAY2TH67W43V5A"); //access key
	let secret_key = String::from("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"); //secret key
	let region = "ap-south-1".parse().unwrap(); //asia pacific mumbai region
	let credentials = Credentials::new(Some(access_key), Some(secret_key), None, None);
	let bucket = Bucket::new(bucket_name, region, credentials);
    	bucket

}

