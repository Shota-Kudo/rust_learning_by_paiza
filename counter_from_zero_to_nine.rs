use std::io; 
fn main(){ 
    let mut n = String::new(); 
    io::stdin().read_line(&mut n).ok(); 
    let mut n: isize = n.trim().parse().expect("0"); 
    for i in 0..10 { 
        println!("{}", n); 
        if n==9 { 
            n = -1; 
        } 
        n+=1; 
    } 
}
