use std::io; 
fn main(){ 
    let mut t = String::new(); 
    io::stdin().read_line(&mut t).ok(); 
    let t: Vec<&str> = t.split_whitespace().collect(); 
    let t1: isize = t[0].trim().parse().expect("0"); 
    let t2: isize = t[1].trim().parse().expect("0"); 
    if t2-t1>0 { 
        println!("+{}", t2-t1); 
    } else if t2-t1<0 { 
        println!("{}", t2-t1); 
    } else if t2-t1==0 { 
        println!("0"); 
    } 
}
