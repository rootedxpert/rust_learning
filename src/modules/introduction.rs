pub fn introduction() {
    println!("");
    print!("Welcome"); // print stays on the same line
    print!("to"); // print stays on the same line
    println!("rust for Dummies"); // println moves to the next line
    println!(
        "i am going to explain all confusing rust concepts you need to know in order to become an effective rust developer, in terms even a js developer can understand"
    );
    println!(
        "in the end not only you will understand rust but you will also start preaching rust to all your co workers"
    );
    println!("");
    println!("part 1: Why rust code and why it doesn't break in production");
    println!("");
    println!("you know some bugs which just show up only in production");
    println!("seg faults,data corruption, race conditions, memory leaks,mysterious crashes, etc");
    println!(
        "rust elimintaes all of these bugs by design at compile time, so you can be sure that your code will not break in production"
    );
    println!("in compile time no garbage collector,no runtime hit");
    eprintln!("rust is a systems programming language, it is fast and memory efficient");
    println!("");
}
