// there are some rules to borrowing, which we will cover in this module
// rust distinct between mutable and immutable references, and it enforecs them in in compile time, so you can be sure that your code will not break in production


struct User {
    name: String,
    age: u32,
}

// rule 1 -> immutable vs mutable references

fn borrowing_1() {
    let user1 = String::from("John");
    let user2 = &user1; // now user1 is still valid, all predetrimed at compile time,under the hood user1 is borrowed to user2, so user1 is still valid
    print_user(&user2);
    print_user(&user1);
} // now user2 and user1 are no longer valid, all predetrimed at compile time

// only one mutable reference is allowed at a time, and it cannot coexist with immutable references, this is to prevent data races
fn borrowing_2() {
    let mut user1 = String::from("John");
    let user2 = &mut user1; // now user1 is no longer valid, all predetrimed at compile time,under the hood user1 is borrowed to user2, so user1 is no longer valid
    user2.push_str(" Doe");
    // as ownership is already passed to above user2 you cant call this function again with user1
    // print_user(&user1);
    print_user(&user2);
} // now user2 and user1 are no longer valid, all predetrimed at compile time

fn borrowing_3() {
    let mut user1 = User {
        name: String::from("John"),
        age: 30,
    };
    print_user(&user1.name);
    // update_name(&user1, String::from("Doe"));
    update_name_v2(&mut user1, String::from("Doe"));
    update_name_v2(&mut user1, String::from("Doe"));
}

fn print_user(user: &String) {
    println!("User: {}", user);
}

fn update_name(user: &User, new_name: String) {
    user.name = new_name; // user.name is behind a & reference
}

fn update_name_v2(user: &mut User, new_name: String) {
    user.name = new_name; // user.name is behind a &mut reference, so we can modify it
}

// rule 2 -> 1 mutable referance and n inmutable references

fn borrowing_4() {
    let mut user1 = User {
        name: String::from("John"),
        age: 30,
    };
    let user2 = &user1; // now user1 is still valid, all predetrimed at compile time,under the hood user1 is borrowed to user2, so user1 is still valid
    let user3 = &user1; // now user1 is still valid, all predetrimed at compile time,under the hood user1 is borrowed to user3, so user1 is still valid
    print_user(&user2.name);
    print_user(&user3.name);
    // as ownership is already passed to above user2 and user3 you cant call this function again with user1
    update_name_v2(&mut user1, String::from("Doe"));
    print_user(&user1.name);
} // now user2 and user3 are no longer valid, all predetrimed at compile time

fn borrowing_5() {
    let mut user1 = User {
        name: String::from("John"),
        age: 30,
    };
    let user2 = &mut user1; // now user1 is still valid, all predetrimed at compile time,under the hood user1 is borrowed to user2, so user1 is still valid
    let user3 = &user1; // now user1 is no longer valid as user1 is already borrowed mutably by user2
    update_name_v2(user2, String::from("Doe"));
    print_user(&user3.name);
} // fix for this is in  next function, we will use scopes to limit the lifetime of the mutable reference

fn borrowing_6() {
    let mut user1 = User {
        name: String::from("John"),
        age: 30,
    };

    let user2 = &mut user1; // now user1 is still valid, all predetrimed at compile time,under the hood user1 is borrowed to user2, so user1 is still valid
    update_name_v2(user2, String::from("Doe"));
    let user3 = &user1; // now user1 is still valid as user2 is no longer valid
    print_user(&user3.name);
} // now user3 is no longer valid, all predetrimed at compile time


// rule 3 -> dangling references, references must be valid 
fn borrowing_7() {
    let user1 = User {
        name: String::from("John"),
        age: 30,
    };
    let user2 = &user1; // now user1 is still valid, all predetrimed at compile time,under the hood user1 is borrowed to user2, so user1 is still valid
    print_user(&user2.name);
} // now user2 and user1 are no longer valid, all predetrimed at compile time

fn borrowing_8() {
    let mut user1 = User {
        name: String::from("John"),
        age: 30,
    };
    update_name_v2(&mut user1, String::from("Doe")); // user1 is moved to update_name_v2, so user1 is no longer valid
    drop(user1); // user1 is dropped here, so user1 is no longer valid
    // print_user(&user1.name); // user1 does not exist anymore
} // now user1 is no longer valid, all predetrimed at compile time

pub fn borrowing() {
    println!();
    println!("module 3: borrowing");
    borrowing_1();
    borrowing_2();
    borrowing_3();
    borrowing_4();
    borrowing_5();
    borrowing_6();
    borrowing_7();
    borrowing_8();
    println!();
}

// we have now seen how rust prevents many classes of bugs that we see in other languages,
// by this simple yet powerful ownership concepts
// bust rust cant prevent logical bugs right? those bugs you crate when you brain is not braining, actually you can to large extent,
// type system is very powerful and expressive, and it can help you catch many logical bugs at compile time, we will cover this in the next module