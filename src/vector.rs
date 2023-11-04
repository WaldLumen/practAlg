use crate::read_input_u32;

pub struct Vector {
   pub x: u32,
   pub y: u32,
}

impl Vector {
    pub fn new(x: u32, y: u32) -> Vector {
        Vector {x,y}
    }

    pub fn length(&self){
    	let length = self.x * self.x + self.y * self.y;
    	let length = (length as f64).sqrt();
    	println!("Длина вектора: {}", length);
    }

    pub fn summa(x1: u32, x2: u32, y1: u32, y2: u32) {
        let x = x1 + x2;
        let y = y1 + y2;
   	println!("Сумма наданих векторів дорівнюе вектору x({}), y({})", x, y);
    }

    pub fn distinction(x1: u32, x2: u32, y1: u32, y2: u32) {
    	let distinction_x = x1 - x2;
    	let distinction_y = y1 - y2;
   	 println!("Разность векторов: {} {}", distinction_x, distinction_y);
    }

    pub fn multiplication(x1: u32, x2: u32, y1: u32, y2: u32) {
    	let multiplication_x = x1 * x2;
    	let multiplication_y = y1 * y2;
    	println!("Умножение векторов: {} {}", multiplication_x, multiplication_y);
}

}

pub fn actions_on_vectors() {
      let vector1 = Vector::new(read_input_u32("Введите х координату", 1000),
				read_input_u32("Введите у координату", 1000));
      let vector2 = Vector::new(read_input_u32("Введите х координату", 1000),
				read_input_u32("Введите у координату", 1000));

      Vector::summa(vector1.x, vector2.x, vector1.y, vector2.y);
      Vector::distinction(vector1.x, vector2.x, vector1.y, vector2.y);
      Vector::multiplication(vector1.x, vector2.x, vector1.y, vector2.y);
      vector1.length();
      vector2.length();
}