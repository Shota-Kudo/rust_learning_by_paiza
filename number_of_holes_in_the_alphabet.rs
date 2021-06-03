use std::io; 
fn main(){ 
    let mut s = String::new(); 
    io::stdin().read_line(&mut s).ok(); 
    let zero = "CEFGHIJKLMNSTUVWXYZ"; 
    let one = "ADOPQR"; 
    for (i, c) in zero.chars().enumerate() { 
        if c.to_string()==s { 
            println!("0"); 
            break; 
        } 
    } 
    for (i, c) in one.chars().enumerate() { 
        if c.to_string()==s { 
            println!("1"); 
            break; 
        } 
    } 
    if s.as_str()=="B" { 
        println!("2"); 
    } 
}
