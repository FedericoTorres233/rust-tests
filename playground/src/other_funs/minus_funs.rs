/// Funtion sub_ten
///
/// # Arguments (num: u32)
/// # Returns u32
///
/// ## Example
/// '''
/// let x = 5;
/// let y = add_five(x);
/// '''

pub fn sub_ten(num: u32) -> u32 {
    return num - 10; // removes 10
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sub_ten_test() {
        let x = 100;
        let y = sub_ten(x);
        println!("x and y is from test: {} {}", x, y);
        assert_eq!(y, 90);
    }
}
