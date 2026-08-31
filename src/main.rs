pub mod modules;
use modules::ownership::main::ownership;
use modules::introduction::introduction;
use crate::modules::borrowing::main::borrowing;
use modules::types::main::types;

fn main() {
    // module 1 introduction
    introduction();
    // module 2 ownership
    ownership();
    // module 3 borrowing
    borrowing();
    // module 4 types
    types();
}
