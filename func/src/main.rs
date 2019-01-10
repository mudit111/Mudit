use std::io;

fn main() {
    let mut n=String::new();
    println!("Enter the number");
    io::stdin().read_line(&mut n).expect("error");
    let m:i32=n.trim().parse().expect("Hello");
    displays(m);
}
fn displays(x:i32){
    for n in 1..x{
        println!("{}",n);
    }
}