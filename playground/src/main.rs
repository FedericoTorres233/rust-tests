mod my_fun;
mod other_funs;

use crate::my_fun::{add_five, add_nine};
use crate::other_funs::minus_funs::sub_ten;

fn main() {
    let mut x: u32 = 50;
    println!("x is {}", x);

    let y: u32 = add_five(x);
    println!("y is {}", y);

    let z: u32 = sub_ten(x);
    println!("z is {}", z);

    x = 70;
    println!("x is {}", x);
}
