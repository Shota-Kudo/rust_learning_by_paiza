use std::io; 
fn main(){ 
    let mut v = String::new(); 
    io::stdin().read_line(&mut v).ok(); 
    let v: Vec<&str> = v.split_whitespace().collect(); 
    let v1: usize = v[0].trim().parse().expect("0"); 
    let v2: usize = v[2].trim().parse().expect("0"); 
    println!("{}", v1+v2); 
}
