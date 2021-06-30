fn sqr(i: i32) -> i32 { i * i }

fn abs(i: i32) -> i32 { if i >= 0 { i } else { -i } }

enum NumberOrNothing {
  Number(i32),
  Nothing
}
use self::NumberOrNothing::{Number,Nothing};

fn number_or_default(n: NumberOrNothing, default: i32) -> i32 {
  match n {
    Nothing => default,
    Number(n) => n,
  }
}

fn compute_stuff(x: i32) -> i32 {
  let y = { let z = x*x; z + 14 };
  y*y
}

fn vec_min(v: Vec<i32>) -> NumberOrNothing {
  fn min_i32(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }     
  }

  let mut min = Nothing;
  for e in v {
    min = Number(match min {      
        Nothing => e,             
        Number(n) => min_i32(n, e)
    });                           
  }
  //@ The `return` keyword exists in Rust, but it is rarely used. Instead, we typically
  //@ make use of the fact that the entire function body is an expression, so we can just
  //@ write down the desired return value.
  min
}

// Inherent implementations
impl NumberOrNothing {
  fn print(self) {
    match self {
      Nothing => println!("The number is: <nothing>"),
      Number(n) => println!("The number is: {}", n),
    };
  }
}

fn read_vec() -> Vec<i32> {
  vec![14, 34, 12, 2, 8, 11, 5]
}

pub fn main() {
  println!("Part O1: Expressions, Inherent methods");
  let vec = read_vec();
  let min = vec_min(vec);
  min.print();
}