extern crate aws;
use s3::bucket::Bucket;
use aws::conn_aws::connection_aws;
use aws::upload::upload_image;

fn main() {
	let bucket: Bucket = connection_aws::make_bucket();
	let response:String = upload_image::upload_s3("/home/knoldus/index.png", "index.png", &bucket);
	println!("{}",response);
}
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_upload() {
		let buc: Bucket = connection_aws::make_bucket();
		assert_eq!(upload_image::upload_s3("/home/knoldus/index.png", "index.png", &buc), "Success");
	}

	/*#[test]
	#[should_panic]
	fn test_upload_fail()
	{
		let buc: Bucket = connection_aws::make_bucket();
		assert_eq!(upload_image::upload_s3("/home/knoldus/index.png", " ", &buc), "Success");
	}*/

	#[test]
	#[should_panic]
	fn test_upload_failure()
	{
		let buc: Bucket = connection_aws::make_bucket();
		assert_ne!(upload_image::upload_s3("/home/knolus/index.png", "index.png", &buc), "Success");
	}

	#[test]
	#[should_panic]
	fn upload_test()
	{
		let buc: Bucket = connection_aws::make_bucket();
		assert_eq!(upload_image::upload_s3("/home/s/index.png", "index.png", &buc), "Error");
	}
}