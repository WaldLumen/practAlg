use crate::read_input_u32;



pub fn ex2_1(){

    let a = read_input_u32("Введите a", 2000) as f64;
    let b = read_input_u32("Введите b", 2000) as f64;
    let z = (((a + b).abs() / (a * b).abs()) + std::f64::consts::PI).sqrt();
    let x= (f64::sin(a.powf(3.0)).powi(2) - f64::asin(1.0 / b)) / f64::ln((a + b).abs()) - 1.0;
    let y = ((x.powi(2))-z.powi(2))/((x-7.0).abs().log10());

    println!("{}", {y})
}

pub fn ex2_2(){


    let all_students = read_input_u32("Введите общее колиечество студентов", 40);
    let all_5_marks = read_input_u32("ВВедите количество студентов с оценкой 5", 40);
    let all_4_marks = read_input_u32("ВВедите количество студентов с оценкой 4", 40);
    let all_3_marks = read_input_u32("ВВедите количество студентов с оценкой 3", 40);
    let all_2_marks = read_input_u32("ВВедите количество студентов с оценкой 2", 40);
    println!("{}% студентов с оценкой 5,\n {}% студентов с оценкой 4,\n {}% студентов с оценкой 3,\n {}% студентов с оценкой 2,\n", {(all_5_marks*100)/all_students}, {(all_4_marks*100)/all_students}, {(all_3_marks*100)/all_students}, {(all_2_marks*100)/all_students})
}