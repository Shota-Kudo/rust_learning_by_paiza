use std::io; 
fn main(){ 
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).ok(); 
    let vec: Vec<&str> = input.split_whitespace().collect(); 
    let d: usize = vec[0].trim().parse().expect("No"); 
    let s: usize = vec[1].trim().parse().expect("No"); 
    if d*1000*100/s >= 10000 { 
        println!("yes"); 
    } else { 
        println!("no"); 
    } 
}
