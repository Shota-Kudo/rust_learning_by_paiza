use std::io; 
fn main(){ 
    let mut name = String::new(); 
    io::stdin().read_line(&mut name).ok(); 
    let vec: Vec<&str> = name.split_whitespace().collect(); 
    let first = vec[0].trim().chars().nth(0).unwrap(); 
    let last = vec[1].trim().chars().nth(0).unwrap(); 
    println!("{}.{}.", first, last); 
}
