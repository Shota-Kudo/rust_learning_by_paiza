use std::io; 
fn main(){ 
    let mut s = String::new(); 
    io::stdin().read_line(&mut s).ok(); 
    let s: f32 = s.trim().parse().expect("0"); 
    println!("{:.1} {:.1}", s-18.0, s-18.5); 
}
