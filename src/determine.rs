use std::io;

pub fn determine_sum_of_firs_2_digits() {
    println!("Введите четырехзначное число:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Не удалось прочитать строку");
    let number: i32 = input.trim().parse().expect("Введите действительное число");

    if number < 1000 || number > 9999 {
        panic!("Введите четырехзначное число.");
    }

    let first_digit = number / 1000;
    let second_digit = (number / 100) % 10;
    let difference = first_digit - second_digit;

    println!("Разница между первыми двумя цифрами: {}", difference);
}
