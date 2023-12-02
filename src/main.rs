mod tools;

use crate::tools::{get_formula, read_input_u32};

mod vector;
use crate::vector::actions_on_vectors;

mod determine;
use crate::determine::determine_sum_of_firs_2_digits;

mod chess;
use crate::chess::chess;
use crate::pract9::figure;

mod chess_tools;

mod pract7;
mod pract9;
mod pract10;


fn main() {
   let formula = get_formula("\n1: Actions on vector\n2: Determine the sum of first two digits.\n3: Chess \n4: Практическая 7.\n5: Практическая 9");

   if formula == 1{
      actions_on_vectors();
   } else if formula ==2 {
     determine_sum_of_firs_2_digits();
   } else if formula == 3 {
      chess();
   } else if formula == 4 {
      pract7::math::ex2_1();
      pract7::math::ex2_2();
   } else if formula == 5 {
      let exercise = get_formula(
         "1: Розробити алгоритм та записати відповідну програму знаходження значення функції, яка обчислюється залежно від значення аргумента.\n\
      2: Розробити алгоритм та записати відповідну програму для поданих нижче завдань.\n\
      3: Розробити алгоритм і програму на одній з алгоритмічних мов, щоб виявити належність точки. М(x, у) геометричній фігурі. Координати точки М та вид фігури наведені нижче.\n\
      4: Прибуток підприємств становить відповідно a, b, c (грошових однинць). Визначити найбільший прибуток підприємств.\n\
      5: Скласти програму, яка визначала б, яких оцінок 4-ок чи 5-ок більше отримано під час іспиту з інформатики");
      if exercise == 1 {
         pract9::math::ex3_1();
      } else if exercise == 2 {
         pract9::math::ex3_2();
      } else if exercise == 3 {
         figure::ex_3_3()
      } else if exercise == 4 {
         pract9::math::ex3_4()
      } else if exercise == 5 {
         pract9::math::ex3_5()
      }
   }
   else if formula == 6 {
      pract10::mnogoottenkovvishmata::maklorens_row_for(1.0, 5);
      pract10::mnogoottenkovvishmata::maklorens_row_while(1.0, 0.01)
   }
}

