use std::io;

pub fn get_formula() -> u32 {
    loop {
        println!("Please input formula what u want use:\n1:Actions on vector\n2:Determine the sum of first two digits.\n3:Chess ");
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
/// * `input_text`:
/// * `_max`:
///
/// returns: u32
///
/// # Examples
///
/// ```
///
/// ```
pub fn read_input_u32(input_text: &str, _max: u32) -> u32 {
    loop {
        println!("{}:", input_text);
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
