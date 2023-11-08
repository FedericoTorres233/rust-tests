// Global variable
const MY_INTEGER: u8 = 10;

fn main() {
    // stack
    let x: u8 = 50;
    println!("x is {}", x);

    // heap
    let mut arr: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    arr.insert(5, 255);
    arr.push(55);
    println!("vec is {:?}", arr);

    // A reference on the stack pointing to a value on the heap
    let arr_2 = &arr[0..3]; // saves a pointer
    println!("arr_2 is {:?}", arr_2);
    println!("arr_3 is {:?}", &arr[0..5]);

    // heap
    let mut s: String = String::from("Federico");
    s.push(' ');
    s.push('!');
    println!("s is {}", s);

    let s_2 = &s[0..3];
    println!("s_2 is {}", s_2);

    println!("MY_INTEGER is {}", MY_INTEGER);
}
