use std::io; 
fn main(){ 
    let mut s = String::new(); 
    io::stdin().read_line(&mut s).ok(); 
    let s: &str = s.as_str(); 
    match s { 
        "Monday"|"Tuesday"|"Wednesday"|"Thursday" => println!("Still {}", s), 
        "Friday" => println!("TGIF"), 
        _ => println!("") 
    } 
}
