use std::io; 
fn main(){ 
    let mut m = String::new(); 
    io::stdin().read_line(&mut m).ok(); 
    let m: usize = m.trim().parse().expect("0"); 
    println!("{}", m+1); 
}
