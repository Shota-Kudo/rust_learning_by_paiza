use std::io; 
fn main(){ 
    let mut s = String::new(); 
    io::stdin().read_line(&mut s).ok(); 
    for (i, c) in s.chars().enumerate() { 
        println!("{}", c); 
    } 
}
