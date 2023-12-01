use std::io;
use crate::read_input_u32;


pub fn ex3_1() {
    println!("Введите x");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let mut y = 0.0;
    let x: f64 = input.trim().parse().expect("Not a valid number");


    if x < -(std::f64::consts::PI / 4.0) {
        y = -(f64::cos(x - std::f64::consts::PI).powi(2));
    } else if x >= -(std::f64::consts::PI / 4.0) && x <= 1.0 {
        y = ((x + 1.0).abs()).sqrt();
    } else if x > 1.0 {
        y = 1.0 / (x - 1.0);
    }

    println!("Y = {}", y);
}

pub fn ex3_2(){
    let mut list: Vec<f64> = vec![];

    let k: f64 = read_input_u32("Введите k", 3000) as f64;
    let l: f64 = read_input_u32("Введите l", 3000) as f64;

    let a: f64 =1.0-(3.0*k) / 5.0;
    let b: f64 = (21.0+k) / k;
    let d: f64 = l*k + 6.5;

    if a > 0.0 {
        list.push(a);
    }
    if b > 0.0 {
        list.push(b);
    }
    if d > 0.0 {
        list.push(d);
    }

    println!("{:?}", {list.clone()});
    let mut id = 0;

    for _ in 1..list.len() + 1 {
         println!("{}", {list[id] * 2.0});
         id += 1;
    }
}


pub fn ex3_4() {

    let mut profit = vec![];

    let mut input = String::new();

    println!("Enter a: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let a: i32 = input.trim().parse().expect("Invalid input");
    profit.push(a);

    input.clear();

    println!("Enter b: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let b: i32 = input.trim().parse().expect("Invalid input");
    profit.push(b);

    input.clear();

    println!("Enter c: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let c: i32 = input.trim().parse().expect("Invalid input");
    profit.push(c);

    let profit_iter = profit.iter();

    println!("Найбильший добуток: {:?}", {profit_iter.max().unwrap()})
}

pub fn ex3_5() {
    let mut input = String::new();

    println!("Enter 5 marks: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let a: i32 = input.trim().parse().expect("Invalid input");

    input.clear();

    println!("Enter 4 marks: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let b: i32 = input.trim().parse().expect("Invalid input");

    if a > b{
        println!("5-ок бiльше")
    } else if b > a{
        println!("4-ок бiльше")
    } else {
        println!("Возникла непредвиденная ошибка, вы можете обратится к автору программы с чётким её описанием")
    }
}