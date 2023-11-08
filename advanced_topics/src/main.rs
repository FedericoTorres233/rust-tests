mod m1_enums;
mod m2_structs;
mod m3_traits;
mod m4_polymorphism;

const NAME: &str = "Rust fede";

fn main() {
    //estatico();
    //dinamico();
    //closures();
    literals()
}

fn estatico() {
    println!("Welcome {}", NAME);

    // stack
    let x: i32;
    x = 2;
    println!("x is {}", x);

    let y: i32 = 4;
    println!("y is {}", y);

    // for loop
    for i in 0..=y {
        if i != 4 {
            print!("{}, ", i);
        } else {
            print!("{} \n", i)
        }
    }

    // Mutation
    let mut z: i32 = 5;
    println!("z was {} ", z);
    z = 10;
    println!("z is now {} ", z);

    let freezing_temp: f64 = -2.7;
    println!("freezing temp is {} ", freezing_temp);

    let is_zero_remainder: bool = 10 % 4 != 0;
    println!("{}", is_zero_remainder);

    let my_char: char = 'z';
    println!("my_char is {}", my_char);

    let emoji: char = '💤';
    println!("emoji is {}", emoji);

    // array in stack
    let my_floats: [f32; 10] = [0.1; 10];
    println!("my_floats is {:?}", my_floats);

    let my_floats_new: [f32; 10] = my_floats.map(|n| n + 2.2);
    println!("my_floats_new is {:?}", my_floats_new);
}

fn dinamico() {
    let name: &str = "Fede";
    println!("my name is {}", name);

    let dynamic_name: String = String::from("Federico");
    println!("dynamic_name is {}", dynamic_name);
    println!("my dynamic_name is stored in memory {:p}", &dynamic_name);

    let dynamic_name: String = name.to_string();
    println!("dynamic_name is {}", dynamic_name);

    let str_slice: &str = &dynamic_name[1..3];
    println!("str_slice is {:?}", str_slice);

    let mut chars: Vec<char> = Vec::new();
    chars.insert(0, 'h');
    chars.insert(1, 'e');
    chars.insert(2, 'l');
    chars.insert(3, 'l');
    chars.push('o');
    chars.push('.');
    println!("chars is {:?}", chars);
    dbg!(&chars);

    let removed_char: char = chars.pop().unwrap();
    dbg!(&removed_char);
    dbg!(&chars);

    chars.iter().for_each(|c: &char| print!("{}", c));
    print!("\n");

    let chars_again: Vec<char> = vec!['h', 'e', 'l', 'l', 'o', '.', '\n'];
    chars_again.iter().for_each(|c: &char| print!("{}", c));

    let collected: String = chars_again.iter().collect();
    print!("{}", collected);

    for c in chars_again {
        print!("{}", c);
        if c == '.' {
            println!(" world!");
        }
    }
}

fn closures() {
    let num: i32 = 5;
    let add_num = |x: i32| x + num;
    let new_num: i32 = add_num(7);
    dbg!(new_num);
}

fn literals() {
    println!("Big Number is {}", 98_222_000_193);
    println!("Hex is {}", 0xfede);
    println!("Octal is {}", 0o7371);
    println!("Binary is {}", 0b1111_1001);
    println!("Bytes 'F' is {}", b'F');

    // Raw - String Literal
    let text: &str = r#"{"message" : "Rust is Awesome"}"#;
    dbg!(text);
}
