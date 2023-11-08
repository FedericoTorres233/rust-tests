fn make_string_not_dangle() -> String {
    let s: String = String::from("not dangle");
    return s;
}

fn main() {
    // Works
    let x: i32 = 50;
    let _y: i32 = x;
    println!("{}", x);

    // Does not work
    //let s: String = String::from("hello");
    //let t: String = s;
    //println!("{}",t)

    // Works
    let s: String = String::from("hello");
    let t: String = s.clone(); // same as 'let t: String = String::from("hello");'
    println!("{}", t);
    
    // Works
    let u: String = String::from("hello2");
    let p: &str = &u;
    let _yy: &str = &u;
    let _l: &str = &u;
    println!("{}", p);
    
    // Works
    let s: String = make_string_not_dangle();
    println!("{}",s);
}
