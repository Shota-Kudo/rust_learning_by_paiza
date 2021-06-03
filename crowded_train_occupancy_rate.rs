use std::io; 
fn main(){ 
    let mut n = String::new(); 
    io::stdin().read_line(&mut n).ok(); 
    let n: usize = n.trim().parse().expect("0"); 
    println!("{}%", n*100/140); 
}
