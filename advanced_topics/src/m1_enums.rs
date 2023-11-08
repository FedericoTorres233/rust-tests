#[derive(Debug)]
enum CarColour {
    Red,
    Green,
    Blue,
    Silver,
}

#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Debug)]
enum GivenOption<T> {
    None,
    Some(T),
}

fn create_car_colour_blue() -> CarColour {
    let my_car_colour: CarColour = CarColour::Blue;
    my_car_colour
}

fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Not under 5!".to_string())
    }
}

fn check_under_five_builtin(num_check: u8) -> Result<u8, String> {
    if num_check < 5 {
        Ok(num_check) // Same as Result::Ok(num_check)
    } else {
        Err("Not under 5!".to_string()) // Same as Result::Err("Not under 5!".to_string())
    }
}

fn remainder_zero(num_check: f32) -> GivenOption<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        GivenOption::Some(remainder)
    } else {
        GivenOption::None
    }
}

fn remainder_zero_builtin(num_check: f32) -> Option<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        Some(remainder) // Same as Option::Some(remainder)
    } else {
        None // Same as Option::None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_enums() {
        let car_colour: CarColour = create_car_colour_blue();
        dbg!(car_colour);

        let under_five_res: GivenResult<u8, String> = check_under_five(2);
        dbg!(under_five_res);

        let remainder_res: GivenOption<f32> = remainder_zero(21.0);
        dbg!(remainder_res);

        let under_five_res_builtin: Result<u8, String> = check_under_five_builtin(7);
        dbg!(under_five_res_builtin);

        let remainder_res_builtin: Option<f32> = remainder_zero_builtin(14.2);
        dbg!(remainder_res_builtin);
    }
}
