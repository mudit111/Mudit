use std::fs;
    pub fn write_to_file(data :String){
        fs::write("/home/knoldus/file2.txt",data).expect("Unable to write the data");
        println!("Successful");

}
