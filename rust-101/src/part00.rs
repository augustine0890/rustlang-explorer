// An `enum` for "a number or nothing" could look as follows:
enum NumberOrNothing {
  Number(i32),
  Nothing
}

fn vec_min(vec: Vec<i32>) -> NumberOrNothing {
  let mut min = NumberOrNothing::Nothing;

  // Iterate over the list
  for el in vec {
    match min {
      // `min` is currently nothing and just make it the number
      NumberOrNothing::Nothing => {
        min = NumberOrNothing::Number(el);
      },

      // compute the new minium and store it
      NumberOrNothing::Number(n) => {
        let new_min = min_i32(n, el);
        min = NumberOrNothing::Number(new_min);
      }
    }
  }
  return min;
}

fn min_i32(a: i32, b: i32) -> i32 {
  if a < b {
    return a;
  } else {
    return b;
  }
}

// The contructors of `NumberOrNothing` into the local namespace
use self::NumberOrNothing::{Number,Nothing};

fn read_vec() -> Vec<i32> {
  vec![14, 34, 12, 2, 8, 11, 5]
}

//@ `println!` is again a macro, where the first argument is a *format string*.
fn print_number_or_nothing(n: NumberOrNothing) {
  match n {
    Nothing => println!("The number is: <nothing>"),
    Number(n) => println!("The number is: {}", n),
  };
}

// Put it all together
pub fn main() {
  let vec = read_vec();
  let min = vec_min(vec);
  print_number_or_nothing(min);
}