use std::fs;
pub fn write_to_string()->String
{
    let data = fs::read_to_string("/home/knoldus/file1.txt").expect("unable to read the data");
    data
}