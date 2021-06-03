use std::io;
fn main(){
    let mut n = String::new();
    io::stdin().read_line(&mut n).ok();
    let n: usize = n.trim().parse().expect("0");
    let  mut a_vec: Vec<usize> = Vec::with_capacity(n);
    for i in 0..n {
        let mut a = String::new();
        io::stdin().read_line(&mut a).ok();
        let a: usize = a.trim().parse().expect("0");
        if a%2!=0 {
            a_vec.push(a);
        }
    }
    let mut tmp:usize = 0; //小さい順に並べるアルゴリズム
    for i in 0..a_vec.len() { //ベクターの要素数を出す
        for j in i+1..a_vec.len() {
            if a_vec[i]>a_vec[j] {
                 tmp = a_vec[i];
                 a_vec[i] = a_vec[j];
                 a_vec[j] = tmp;
            }
        }
        println!("{}", a_vec[i]);
    }
}
