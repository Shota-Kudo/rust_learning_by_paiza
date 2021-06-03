use std::io; 
fn main(){ 
    let mut s = String::new(); 
    io::stdin().read_line(&mut s).ok(); 
    let mut str = "".to_string(); 
    for (i, c) in s.chars().enumerate() { 
        if i<s.chars().count()-8 { 
            str.push(c); 
        } 
    } 
    println!("{}!!", str,); 
}
