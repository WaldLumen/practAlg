use std::io;
use std::io::Write;

pub fn get_formula(message_text: &str) -> u32 {
    loop {
        println!("{}", {message_text});
        print!("Please input formula what u want use: ");
        io::stdout().flush().unwrap();

        let mut formula = String::new();

        io::stdin()
            .read_line(&mut formula)
            .expect("Failed to read line");

        let formula: u32 = match formula.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break formula;
    }
}

///
///
/// # Arguments
///
/// * `input_text`: string
/// * `_max`(maximum possible number value): int
///
/// returns: u32
///
/// # Examples
///
/// ```
/// let k: u32 = read_input_u32("Введите k:", 3000);
/// ```
pub fn read_input_u32(input_text: &str, _max: u32) -> u32 {
    loop {
        print!("{}: ", input_text);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");
        match input.trim().parse() {
            Ok(num) => {
                if num < _max {
                    break num;
                } else {
                    println!("Ваше число больше наивысшего доступного - {}", _max);
                }
            }
            Err(_) => {
                println!("Не удалось преобразовать в число.");
                continue;
            }
        }
    }
}
