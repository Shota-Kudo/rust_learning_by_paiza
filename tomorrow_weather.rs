use std::io; 
fn main(){ 
    let mut n = String::new(); 
    io::stdin().read_line(&mut n).ok(); 
    let N: usize = n.trim().parse().expect("No"); 
    //println!("{}", N); 
     
    let mut d: Vec<String> = Vec::with_capacity(N); 
    for x in 0..N { 
        let mut D = String::new(); 
        io::stdin().read_line(&mut D).ok(); 
        d.push(D); 
        if d[x].trim()=="forward" { 
            println!("Sunny"); 
        }else if d[x].trim()=="reverse" { 
            println!("Rainy"); 
        }else if d[x].trim()=="sideways" { 
            println!("Cloudy"); 
        } 
        //println!("{:?}", d); 
    }   
}
