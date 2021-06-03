use std::io; 
fn main(){ 
    let mut f = String::new(); 
    io::stdin().read_line(&mut f).ok(); 
    let f: isize = f.trim().parse().expect("0"); 
    if 20<=f && f<=15000 { 
        println!("yes"); 
    } else if 15000<f && f<=20000 { 
        println!("not sure"); 
    } else { 
        println!("no"); 
    } 
}
