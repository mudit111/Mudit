use std::io;

fn main() {
    let mut n=String::new();
    println!("Enter the number");
    io::stdin().read_line(&mut n).expect("error");
    let m:i32=n.trim().parse().expect("Hello");
    displays(m);
}
fn displays(x:i32){
    let n=1..x;
    for i in n{
        println!("{}",i);
    }
}