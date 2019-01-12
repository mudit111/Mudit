use s3::{Bucket,Credentials,Region};

pub fn connection() ->Bucket{

    //here we store the bucket name
    let name: &str = "hawktestbucket";
    //here we select the region correspondent to our select region while creatiion of the s3 bucket
    // let region:Region = "ap-south-1".parse().expect("unable to validate region");
    let region: Region = Region::ApSouth1;
    //it reads and loads the credentials from the environment where access key and secret key id provided by us and
    //we initialize the credentials directly with the new function
    let credentials: Credentials = Credentials::new("AKIAIHW6UQ53LA5LJOQA", "TxwID2zpkpWBnXSYIfbVUKgNAntHWYVNhfCpPpi5", None);
    //here we use the new fucntion of the bucket and create the instance of the bucket
    let bucket: Bucket = Bucket::new(name, region, credentials);//bucket instantiation

    bucket//return bucket instance

}