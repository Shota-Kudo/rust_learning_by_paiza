use std::io;
fn main(){
    let mut n = String::new();
    io::stdin().read_line(&mut n).ok();
    let n: usize = n.trim().parse().expect("0");
    let mut c_vec: Vec<String> = Vec::with_capacity(n);
    let mut flag = 0;

    for i in 0..n {
        let mut c = String::new();
        io::stdin().read_line(&mut c).ok();
        let c = c.trim().to_string();
        if i==0 {   //最初の値を挿入
            println!("YES");
            c_vec.push(c);
            continue;
        }
        
        for j in 0..i {    //値の検索
            if c==c_vec[j] {
                flag = 1;
                break;
            }
        }
        
        if flag==1 {
            println!("NO");
            c_vec.push(c);
            flag = 0;
        } else if flag==0 {
            println!("YES");
            c_vec.push(c);
        }
    }
}
