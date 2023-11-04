use crate::read_input_u32;


use crate::chess_tools::{queen_actions, elephant_actions, is_piece_within_moves};
pub struct Chess {
    pub x: u32,
    pub y: u32,
    pub possible_moves: Vec<Vec<u32>>,
}

impl Chess {
    pub fn new(x: u32, y: u32, possible_moves: Vec<Vec<u32>>) -> Chess {
        Chess { x, y, possible_moves }
    }
}

pub fn add_white_king() -> Chess {
    let wkx = read_input_u32("Введите х координату белого короля", 8);
    let wky = read_input_u32("Введите y координату белого короля", 8);

    let white_king_actions: Vec<Vec<u32>> = vec![
        vec![wkx + 1, wky],
        vec![wkx - 1, wky],
        vec![wkx, wky + 1],
        vec![wkx, wky - 1],
        vec![wkx + 1, wky + 1],
        vec![wkx + 1, wky - 1],
        vec![wkx - 1, wky - 1],
        vec![wkx - 1, wky + 1],
    ];

    return Chess::new(wkx, wky, white_king_actions.clone());
}

pub fn add_white_elephant() -> Chess {
    let wex = read_input_u32("Введите х координату белого слона", 8);
    let wey = read_input_u32("Введите y координату белого слона", 8);

    let actions: Vec<Vec<u32>> = elephant_actions(wex, wey, vec![]);

    return Chess::new(wex, wey, actions);
}


pub fn add_black_queen() -> Chess {
    let wex = read_input_u32("Введите х координату чёрной королевы", 8);
    let wey = read_input_u32("Введите y координату чёрной королевы", 8);

    let actions: Vec<Vec<u32>> = queen_actions(wex, wey, vec![]);

    return Chess::new(wex, wey, actions);
}

pub fn chess(){
    let white_elephant = add_white_elephant();
    let white_king = add_white_king();
    let black_queen = add_black_queen();

    is_piece_within_moves(black_queen.possible_moves.clone(), white_king.x, white_king.y, "Black Queen", "White King", "Attack");
    is_piece_within_moves(black_queen.possible_moves.clone(), white_elephant.x, white_elephant.y, "Black Queen", "White Elephant", "Attack");
    is_piece_within_moves(white_king.possible_moves.clone(), white_elephant.x, white_elephant.y, "White King", "White Elephant", "Defence");
    is_piece_within_moves(white_king.possible_moves.clone(), black_queen.x, black_queen.y, "White King", "Black Queen", "Attack");
    is_piece_within_moves(white_elephant.possible_moves.clone(), white_king.x, white_king.y, "White Elephant", "White King", "Defence");
    is_piece_within_moves(white_elephant.possible_moves.clone(), black_queen.x, black_queen.y, "White Elephant", "Black Queen", "Attack");

}
