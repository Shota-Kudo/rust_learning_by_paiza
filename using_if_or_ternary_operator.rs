use std::io; 
fn main(){ 
    let mut input: Vec<String> = Vec::with_capacity(5); 
    for i in 0..5 { 
        let mut s = String::new(); 
        io::stdin().read_line(&mut s).ok(); 
        input.push(s); 
    } 
     
    let a: isize = input[0].trim().parse().expect("0"); 
    if a>0 { 
        println!("plus"); 
    } else { 
        println!("{}", a); 
    } 
     
    if input[1].as_str().trim()=="hoge" { 
        println!("yes");     
    } else { 
        println!("{}", input[1].trim()); 
    } 
     
    if input[2].chars().count()-1==10 { 
        println!("ten"); 
    } else { 
        println!("{}", input[2].trim()); 
    } 
     
    for (i, c) in input[3].chars().enumerate() { 
        if c.to_string()=="x" { 
            println!("{}", i); 
            break; 
        } else if i+1==input[3].trim().chars().count(){ 
            println!("nothing"); 
        } 
    } 
     
    if input[4].trim().chars().count()== 5 { 
        println!("five"); 
    } else { 
        println!("{}", input[4].trim().chars().nth(0).unwrap()); 
    } 
}
