fn change_string(text: &mut String){
    text.push('?')
}

fn main() {
    
    // Works
    let mut s: String = String::from("hello");
    let t: &mut String = &mut s;
    t.push('!');
    change_string(t);
    println!("{}", t);
    println!("{}", s);
}
