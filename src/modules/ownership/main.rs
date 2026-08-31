// ownership is a set of rules that governs how a rust program manages memory
//When a variable goes out of scope, the compiler automatically inserts calls to the variable's destructor, implemented via the Drop trait, which handles the deallocation.
fn ownership_1() {
    let user1 = String::from("John");
    println!("User1: {}", user1);
} // now user1 is no longer valid, all predetrimed at compile time

fn ownership_2() {
    let user1 = String::from("John");
    let user2 = user1; // now user1 is no longer valid, all predetrimed at compile time,under the hood user1 is moved to user2, so user1 is no longer valid
    println!("User2: {}", user2);
} // now user2 is no longer valid, all predetrimed at compile time

fn ownership_3() {
    let user1 = String::from("John");
    let user2 = &user1; // now user1 is still valid, all predetrimed at compile time,under the hood user1 is borrowed to user2, so user1 is still valid
    println!("User2: {}", user2);
    println!("User1: {}", user1);
} // now user2 and user1 are no longer valid, all predetrimed at compile time

fn ownership_4() {
    let user1 = String::from("John");
    let user2 = user1; // now user1 is still valid, all predetrimed at compile time,under the hood user1 is borrowed to user2, so user1 is still valid
    print_user(user2);
    // as ownership is already passed to above print_user you cant call this function again with user2
    print_user(user2);
    // now user2 is no longer valid, all predetrimed at compile time as ownership is moved to the function print_user, so user2 is no longer valid
    // this eliminates the variaity of bugs that we see in other langugaes
    // double free, dangling pointer, memory leak, etc
    println!("User1: {}", user2);
}

fn print_user(user: String) {
    println!("User: {}", user);
}

// ownership is just a piece of the puzzle,

// rust has clever way to resolve all this issue with something know as borrowing,
// which allows you to have multiple references without taking ownership of the data,there are some rules to borrowing, which we will cover in the next module

pub fn ownership() {
    println!();
    println!("module 2: ownership");
    ownership_1();
    ownership_2();
    ownership_3();
    ownership_4();
}
