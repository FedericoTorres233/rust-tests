fn main() {
    // Stored in heap
    let s: String = String::from("Hello!");
    let s_2: &str = &s[0..5];
    println!("s_2 is {}", s_2);

    // Stored in stack
    let msg: &str = "hello2";
    println!("msg is {}", msg);

    let msg_string: String = "hello3".to_string();
    println!("msg_string is {}", msg_string);
}
