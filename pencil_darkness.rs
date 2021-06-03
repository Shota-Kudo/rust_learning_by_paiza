use std::io; 
fn main(){ 
    let mut k = String::new(); 
    io::stdin().read_line(&mut k).ok(); 
    let k: usize = k.trim().parse().expect("0");  
    let concentration: [&str;17] = ["6B", "5B", "4B", "3B", "2B", "B", "HB", 
    "F", "H", "2H", "3H", "4H", "5H", "6H", "7H", "8H", "9H"]; 
    println!("{}", concentration[k-1]); 
}
