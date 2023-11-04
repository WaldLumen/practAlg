mod tools;
use crate::tools::{get_formula, read_input_u32};

mod vector;
use crate::vector::actions_on_vectors;

mod determine;
use crate::determine::determine_sum_of_firs_2_digits;

mod chess;
use crate::chess::chess;
mod chess_tools;

fn main() {

   let formula = get_formula();
   
   if formula == 1{
      actions_on_vectors();
   } else if formula ==2 {
     determine_sum_of_firs_2_digits();
   } else if formula == 3 {
      chess();
   }
   
}