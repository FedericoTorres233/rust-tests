fn main() {
    let mut s: String = String::from("Fede");
    let t: &mut String = &mut s;
    
    *t = String::from("Federico");
    println!("{}",s);
    
    
    let mut x: i32 = 50;
    x = 70;
    dbg!(x);
    
    let y: &mut i32 = &mut x;
    *y += 1;
    dbg!(x);
}
