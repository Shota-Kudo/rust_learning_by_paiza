fn main(){
    let mut hw = String::new();
    std::io::stdin().read_line(&mut hw).ok();
    let hw: Vec<&str> = hw.split_whitespace().collect();
    let h: usize = hw[0].trim().parse().expect("0");
    let w: usize = hw[1].trim().parse().expect("0");
    
    let mut t_vec: Vec<String> = Vec::with_capacity(h);
    for i in 0..h {
        let mut t = String::new();
        std::io::stdin().read_line(&mut t).ok();
        t_vec.push(t);
    }
    
    println!("{}", t_vec[0]);
}
