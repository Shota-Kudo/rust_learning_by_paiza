fn main(){
    let mut y = String::new();
    std::io::stdin().read_line(&mut y).ok();
    let y: usize = y.trim().parse().expect("0");
    if y%4==0 {
        if y%100==0 {
            if y%400==0 {
                println!("Yes");
                return ;
            }
            println!("No");
            return ;
        }
        println!("Yes");
    } else {
        println!("No");
    }
}
