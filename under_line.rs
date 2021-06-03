use std::io; 
fn main(){ 
    let mut s = String::new(); 
    io::stdin().read_line(&mut s).ok(); 
    println!("{}", s); 
    for i in 0..s.chars().count() { 
        print!("^"); 
    } 
}
