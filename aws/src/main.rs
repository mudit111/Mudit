use s3::credentials::Credentials;
use s3::bucket::Bucket;
use std::fs::File;
use std::io::Read;

fn main() {
    
    	let bucket: Bucket = connection();
	image_upload("/home/knoldus/index.png", "index.png", &bucket);
}

pub fn image_upload(path: &str, file_name :&str, bucket :&Bucket){

    	let mut file_content:Vec<u8> = Vec::new();
    	let mut file: File = File::open(path).expect("incorrect data");
    	file.read_to_end(&mut file_content).unwrap();
    	//bucket.put(file_name,file_content.as_slice(),"image/png").expect("no file found");
    	bucket.put("index.png",file_content.as_slice(),"image/png").expect("no file found");
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
