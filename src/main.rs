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
        name: String, // String owns its data (unlike &str which borrows)
        age: u32,
        scores: Vec<u32>, // A dynamic, growable list of numbers
    }
    impl Student {
        // Constructor function. Notice it takes `name: String`.
        // This means the function takes FULL OWNERSHIP of the string passed into it.
        fn new(id: u32, name: String, age: u32) -> Student {
            Student {
                id,
                name,
                age,
                scores: Vec::new(), // Starts with an empty vector
            }
        }

        // Mutable borrow (&mut self): We need to modify the student's vector.
        fn add_score(&mut self, score: u32) {
            self.scores.push(score);
        }

        // Immutable borrow (&self): We only need to read the scores to sum them.
        fn total_score(&self) -> u32 {
            self.scores.iter().sum()
        }

        // Immutable borrow (&self)
        fn average(&self) -> f32 {
            if self.scores.is_empty() {
                return 0.0;
            }
            self.total_score() as f32 / self.scores.len() as f32
        }

        // Immutable borrow (&self)
        fn grade(&self) -> Grade {
            let avg = self.average();
            // Pattern matching for numbers!
            match avg {
                a if a >= 90.0 => Grade::Excellent,
                a if a >= 75.0 => Grade::VeryGood,
                a if a >= 60.0 => Grade::Good,
                a if a >= 50.0 => Grade::Pass,
                _ => Grade::Fail,
            }
        }

        // Immutable borrow (&self)
        fn display(&self) {
            println!("--- STUDENT REPORT ---");
            println!("ID:      {}", self.id);
            println!("Name:    {}", self.name);
            println!("Age:     {}", self.age);
            println!("Scores:  {:?}", self.scores);
            println!("Total:   {}", self.total_score());
            println!("Average: {:.2}", self.average());
            println!("Grade:   {:?}", self.grade());
            println!("----------------------");
        }
    }
    // Helper to get a String from the user
    fn get_string(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap(); // Ensures the prompt prints before asking for input

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        input.trim().to_string() // .trim() removes the newline character (Enter key)
    }

    // Helper to get a valid number from the user
    fn get_u32(prompt: &str) -> u32 {
        loop {
            let input = get_string(prompt);
            // We use pattern matching to handle the Result of parsing
            match input.parse::<u32>() {
                Ok(num) => return num, // If successful, exit the loop and return the number
                Err(_) => println!("Invalid input. Please enter a valid number."),
            }
        }
    }
    fn main() {
        // Here is our in-memory database!
        // It must be `mut` because we will add and remove students.
        let mut students: Vec<Student> = Vec::new();

        loop {
            println!("\n=== STUDENT MANAGEMENT SYSTEM ===");
            println!("1. Add Student");
            println!("2. Add Score to Student");
            println!("3. View All Students");
            println!("4. View Single Student Report");
            println!("5. Remove Student");
            println!("6. Exit");

            let choice = get_u32("Enter your choice: ");

            match choice {
                1 => {
                    let id = get_u32("Enter Student ID: ");

                    // Bonus: Prevent duplicate IDs by checking if any student already has this ID
                    if students.iter().any(|s| s.id == id) {
                        println!("Error: Student with ID {} already exists!", id);
                        continue; // Skips the rest of this loop iteration and shows the menu again
                    }

                    let name = get_string("Enter Student Name: ");
                    let age = get_u32("Enter Student Age: ");

                    // name is moved (ownership transferred) into the struct here
                    let new_student = Student::new(id, name, age);
                    students.push(new_student);
                    println!("Student added successfully!");
                }
                2 => {
                    let id = get_u32("Enter Student ID: ");

                    // We need a MUTABLE reference to the student to change their scores.
                    // .iter_mut() allows us to borrow the vector items mutably.
                    if let Some(student) = students.iter_mut().find(|s| s.id == id) {
                        let score = get_u32("Enter Score to add: ");
                        student.add_score(score);
                        println!("Score added successfully!");
                    } else {
                        println!("Error: Student not found.");
                    }
                }

                3 => {
                    if students.is_empty() {
                        println!("No students in the system.");
                    } else {
                        println!(
                            "{:<5} | {:<15} | {:<5} | {:<10} | {:<5}",
                            "ID", "Name", "Age", "Subjects", "Average"
                        );
                        println!("{:-<50}", ""); // Prints a dividing line

                        for s in &students {
                            // Borrowing the vector immutably
                            println!(
                                "{:<5} | {:<15} | {:<5} | {:<10} | {:.2}",
                                s.id,
                                s.name,
                                s.age,
                                s.scores.len(),
                                s.average()
                            );
                        }
                    }
                }

                4 => {
                    let id = get_u32("Enter Student ID to view: ");

                    // Immutable borrow to find and display
                    if let Some(student) = students.iter().find(|s| s.id == id) {
                        student.display();
                    } else {
                        println!("Error: Student not found.");
                    }
                }
                5 => {
                    let id = get_u32("Enter Student ID to remove: ");

                    let initial_len = students.len();
                    // .retain() keeps only elements that match the condition.
                    // If the ID matches, it returns false, dropping that student from the vector.
                    students.retain(|s| s.id != id);

                    if students.len() < initial_len {
                        println!("Student removed successfully!");
                    } else {
                        println!("Error: Student not found.");
                    }
                }

                6 => {
                    println!("Exiting program");
                    break; // This is the ONLY way out of the loop!
                }

                _ => println!("Invalid choice. Please select 1-6."),
            }
        }
    }
}
