pub enum Option<T> {
    None,
    Some(T),
}

pub struct User {
    name: String,
}

pub struct UserProfile {
    name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        User { name }
    }

    pub fn rename(&mut self, new_name: String) {
        self.name = new_name;
    }

    pub fn from_user_profile(user_profile: &UserProfile) -> Self {
        User {
            name: user_profile.name.clone(),
        }
    }
}
// to handle the same in rust like null pointer exception we have to do nothing
// in rust we have something called option type,now compiler will force you to handle the case when the value is not present, so you can avoid null pointer exception
// additionaly values in rust are inmutable by default, so you can avoid accidental mutation of values, which is a common source of bugs in other languages
pub fn types() {
    println!();
    println!("module 4: types");
    let user1 = User::new(String::from("John"));
    println!("User1: {}", user1.name);
    let mut user2 = User::new(String::from("Doe"));
    println!("User2: {}", user2.name);
    user2.rename(String::from("Smith"));
    println!("User2: {}", user2.name);
    let user_profile = UserProfile {
        name: String::from("Jane"),
    };
    let user3 = User::from_user_profile(&user_profile);
    println!("User3: {}", user3.name);
}