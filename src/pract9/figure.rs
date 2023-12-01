use std::io;
use std::io::Write;
use crate::tools::read_input_u32;

type Dot = (String, Vec<u32>);

pub struct Figure{
    r: u32,
    zero: u32,
    dots: Vec<(String, Vec<u32>)>
}

impl Figure {
    pub fn new(r: u32, zero: u32, dots: Vec<(String, Vec<u32>)>) -> Figure {
        Figure { r, zero, dots }
    }
    ///
    ///
    /// # Arguments
    ///
    /// * `dots`: Vec<(str, Vec<u32>)>
    /// * `index`: u32(where 0 - x; 1 - y)
    ///
    /// returns: u32
    ///
    /// # Examples
    ///
    /// ```
    /// let limit = figure::Figure::get_limit_dots(list, 0);
    /// ```
    pub fn get_limit_dots(dots: Vec<(String, Vec<u32>)>, index: usize) -> Option<u32> {
        // Итерирование по массиву и нахождение максимального значения
        let max_value = dots
            .iter()
            .flat_map(|(_, coordinates)| coordinates.get(index).cloned())
            .max();

        max_value
    }
    pub fn all_dots_occupied_by_the_figure(lim_x_max:u32, lim_x_min: u32, lim_y_max: u32, lim_y_min: u32) -> Vec<Vec<u32>> {
        let mut element: Vec<Vec<u32>> = vec![];
        let mut x = lim_x_min;
        let mut y = lim_y_min;

        loop{
            if x < lim_x_max && y < lim_y_max {
                x += 1;
                y += 1;
                element.push(vec![x, y])
            } else if x < lim_x_max || y > lim_y_max {
                x += 1;
                element.push(vec![x, y])
            } else if x > lim_x_max || y < lim_y_max{
                y += 1;
                element.push(vec![x, y])
            } else {
                break;
            }
        }
        return element;
    }
}

    pub fn get_figure_dots(mut dots: Vec<Dot>) -> Vec<(String, Vec<u32>)> {
        loop {
            print!("Input Dot name: ");
            io::stdout().flush().unwrap();
            let mut dot_name = String::new();
            io::stdin().read_line(&mut dot_name).expect("Not a valid string");
            let dot_name = dot_name.trim();

            if dot_name == "end" {
                println!("You have finished entering all the dots");
                break
            }

            println!("Input the {} Dot coords", { dot_name });

            let dot_x_coord = read_input_u32("Введите x координату точки", 11);
            let dot_y_coord = read_input_u32("Введите y координату точки", 11);

            dots.push((dot_name.to_string(), vec![dot_x_coord, dot_y_coord]));
        }
        return dots;
    }


pub fn ex_3_3() {
    let mut input = String::new();

    println!("Enter r: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let r: f32 = input.trim().parse().expect("Invalid input");

    input.clear();

    println!("Enter x: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let x: f32 = input.trim().parse().expect("Invalid input");

    input.clear();

    println!("Enter y: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let y: f32 = input.trim().parse().expect("Invalid input");

    if x <= r && y <=r {
        println!("точка належить фигуре");
    } else {
        println!("точка не налкежить фигуре");
    }
}
