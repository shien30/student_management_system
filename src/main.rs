fn main() {
    use std::io::{self, Write};

// 1. Grade Enum
// We use #[derive(Debug)] so we can easily print the enum variants using {:?}
#[derive(Debug)]
enum Grade {
    Excellent,
    VeryGood,
    Good,
    Pass,
    Fail,
}

// 2. Student Struct
struct Student {
    id: u32,
    name: String,   // String owns its data (unlike &str which borrows)
    age: u32,
    scores: Vec<u32>, // A dynamic, growable list of numbers
}

   