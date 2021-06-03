use std::io;
fn fizzbuzz(n: usize){
    for i in 0..n {
        if i%15==0 {
            println!("FizzBuzz");
        }else if i%3==0 {
            println!("Fizz");
        }else if i%5==0 {
            println!("Buzz");
        }else{
            println!("{}", i);
        }
    }
}

fn square_sum(n: isize) -> isize {
    (0..n).filter(|i| i%2==0).map(|i| i*i).sum()
}


fn rebind() {
    let sum = 0;
    for i in 0..10 {
        let sum = sum + i;
    }
    println!("{}", sum);
}

fn reassign() {
    let mut sum = 0;
    for i in 0..10 {
        sum = sum + i;
    }
    println!("{}", sum);
}


fn add(x: isize, y: isize) -> isize {
    x + y
}

fn factorial(n: usize) -> usize {
    if n==0 {
        1
    } else {
        n * factorial(n-1)
    }
}

fn print_string(s: String) {
    println!("{}", s);
}

fn ref_string(s: &String) {
    println!("{}", s);
}

fn refmut_string(s: &mut String) {
    println!("{}", s);
}

fn pair<T, S>(t:T, s:S) -> (T, S){(t, s)}

struct Dummy;

struct Point(f64, f64);

struct Color {
    r:u8,
    g:u8,
    b:u8,
}

struct Celcius(f64);
struct Kelvin(f64);

impl Celcius {
    fn to_kelvin(self) -> Kelvin {
        Kelvin(self.0 + 273.15)
    }
    fn from_kelvin(k:Kelvin) -> Self {
        Celcius(k.0 - 273.15)
    }
}

enum Command {
    Right(i64),
    Up(i64),
    Move {x: i64, y: i64},
    Print,
}

trait DuckLike {
    fn quack(&self);
    fn walk(&self){
        println!("walking");
    }
}

struct Duck;

impl DuckLike for Duck {
    fn quack(&self) {
        println!("quack");
    }
}

struct Tsuchinoko;

impl DuckLike for Tsuchinoko {
    fn quack(&self) {
        println!("mew");
    }
    fn walk(&self) {
        println!("wriggling");
    }
}

impl DuckLike for i64 {
    fn quack(&self) {
        for _ in 0..*self {
            println!("quack");
        }
    }
}

fn main(){
    let duck = Duck;
    let tsuchinoko = Tsuchinoko;
    let i = 3;
    duck.quack();
    tsuchinoko.quack();
    i.quack();
    duck.walk();

    /*let mut cur = (0, 0);
    let commands = &[Command::Move {x: 0, y: 0},
                    Command::Right(5),
                    Command::Up(5),
                    Command::Print,
                    Command::Move {x: 10, y:10},
                    Command::Print];
    for c in commands {
        match *c {
            Command::Right(x) => cur.0 += x,
            Command::Up(y) => cur.1 += y,
            Command::Move {x, y} => {
                cur.0 = x;
                cur.1 = y;
            }
            Command::Print => {
                println!("{:?}", cur);
            }
        }
    }*/

    /*let absolute_zero = Kelvin(0.0);
    let triple_point = Celcius(0.0);
    let celcius = Celcius::from_kelvin(absolute_zero);
    let kelvin = triple_point.to_kelvin();*/

    /*let dummy = Dummy;
    let point = Point(0.0, 0.0);
    let x = point.0;
    let black = Color {r:0, g:0, b:0};
    let r = black.r;*/

    /*let i = pair(1, 1.0);
    let i = pair::<isize, f64>(1, 1.0);
    let s = pair("str", "String".to_string());*/

    /*let x = 1;
    let y = x;
    println!("{:?}", y);
    println!("{}", y);
    let a = "abc";
    let b = a;
    println!("{}", a);*/

    /*let s = "owned data".to_string();
    {
        let t = s;
    }
    {
        let s = "owned data".to_string();
        let ref_s = &s;
    }*/
    
    /*let mut vec = vec![1, 2, 3];
    for i in vec.iter() {
        vec.push(i*2);
    }*/

    /*let mut s = "there is resource".to_string();
    //let t = &s;
    let t = &mut s;
    ref_string(&s);
    refmut_string(&s);*/

    /*let mut n = String::new();
    io::stdin().read_line(&mut n).ok();
    let n: usize = n.trim().parse().expect("0");
    let mut c_vec: Vec<String> = Vec::with_capacity(n);
    //println!("{}", c_vec[0]);
    let mut flag = 0;
    for i in 0..n {
        let mut c = String::new();
        io::stdin().read_line(&mut c).ok();
        let c = c.trim().to_string();
        //println!("{:?}", c_vec);
        if i==0 {//最初の値を挿入
            println!("YES");
            c_vec.push(c);
            continue;
        }
        
        for j in 0..i {//値の検索
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
    }*/

    /*let mut AB = String::new();
    io::stdin().read_line(&mut AB).ok();
    let AB: Vec<&str> = AB.split_whitespace().collect();
    let A:f64 = AB[0].trim().parse().expect("0");
    let B:f64 = AB[1].trim().parse().expect("0");
    let AB: f64 = (B-A+1.0)*(A+B)/2.0;
    println!("{}", AB);*/

    /*let s = "there is resource".to_string();
    let t = s;
    print_string(t);*/

    /*factorial(10);
    let x = let y = 1;*/ //コンパイルエラー　文の右辺に文はおけない
    
    /*match 10 {
        0 => println!("0"),
        1...10 => println!("small number"),
        n => println!("big number: {}", n)
    }
    match (1.0, 1) {
        (0.0, 0) => println!("all zero"),
        (f, 0...10) => println!("float: {} with small number", f),
        _ => println!("other tuple")
    } */  

    /*let a: (isize, f64, &str) = (1, 1.0, "abc");
    println!("{},{},{}", a.0, a.1, a.2);
    println!("{}", add(1, 2));
    let f: fn(isize, isize) -> isize = add;
    let b = f(1, 2);
    println!("{}", b);*/
    
    /*let mut a: String = "abc".to_string();
    a += "def";
    println!("{}", a);
    let x = 1.0.to_string();
    println!("{}", x);
    a += x.as_str();
    println!("{}", a);*/

    /*let a: [isize;3] = [1, 2, 3];
    let b: &[isize] = &a;
    println!("{:?}", b);
    for elm in b {
        println!("{}", elm);
    }
    println!("{:?}", b[0]);*/

    /*let x = 1;
    let y: &isize = &x;
    let mut a = 1;
    let b = &mut a;
    *b = 2;
    println!("{}", *b);*/
    
    //fizzbuzz(20);
    //println!("Hello,World!");
    //rebind();
    //reassign();
    //println!("{}", square_sum(10));
    /*let  x: i32 = 1+2;
    let mut y = x;
    let x: &str = "abc";
    y=5;
    let z = y;
    println!("{},{}", x, z);*/
}
