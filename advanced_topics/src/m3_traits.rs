trait Attacker {
    fn choose_style(&self) -> String;
}

#[derive(Debug)]
enum Character {
    Warrior,
    Archer,
    Wizard,
}

impl Attacker for Character {
    fn choose_style(&self) -> String {
        // v---------- More less like a switch
        match self {
            Character::Warrior => "A las piñas".to_string(),
            Character::Archer => "Un palo y una flecha".to_string(),
            Character::Wizard => "Pium pium".to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_traits() {
        let my_character: Character = Character::Warrior;
        let chosen_style = my_character.choose_style();
        dbg!(chosen_style);
    }
}
