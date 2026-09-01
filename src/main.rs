pub mod modules;
use crate::modules::borrowing::main::borrowing;
use modules::generics::main::generics;
use modules::introduction::introduction;
use modules::ownership::main::ownership;
use modules::traits::main::traits;
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
    // module 5 generics
    generics();
    // module 6 trait
    traits();
}
