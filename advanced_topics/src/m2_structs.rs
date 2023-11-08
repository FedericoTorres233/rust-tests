#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn increment_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }

    fn change_email(&mut self, new_email: &str) {
        self.email = String::from(new_email)
    }
}

fn change_username(user: &mut User, new_username: &str) {
    user.username = String::from(new_username)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_structs() {
        let mut user1: User = User {
            username: String::from("someusername1"),
            email: String::from("someemail1"),
            sign_in_count: 4,
            active: true,
        };
        let user1_ref = &mut user1;
        change_username(user1_ref, "holaaa");
        dbg!(user1);

        let mut user2: User = User {
            username: String::from("someusername2"),
            email: String::from("someemail2"),
            sign_in_count: 10,
            active: false,
        };
        user2.increment_sign_in_count();
        user2.change_email("fede@something.com");
        dbg!(user2);
    }
}
