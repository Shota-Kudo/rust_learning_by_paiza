use std::io; 
fn main(){ 
    let mut I = String::new(); 
    io::stdin().read_line(&mut I).ok(); 
    let I: isize = I.trim().parse().expect("0"); 
    if I<30 { 
        println!("quiet"); 
    } else if 30<=I && I<50 { 
        println!("normal"); 
    } else if 50<=I && I<70 { 
        println!("noisy"); 
    } else if 70<=I { 
        println!("very noisy"); 
    } 
}
