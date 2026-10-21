use std::fs::File;
use std::io::Write;

struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() {
    // Create a vector containing student details
    let students = vec![
        Student {
            name: String::from("Oluchi Mordi"),
            matric_number: String::from("ACC10211111"),
            department: String::from("Accounting"),
            level: 300,
        },
        Student {
            name: String::from("Adams Aliyu"),
            matric_number: String::from("ECO10110101"),
            department: String::from("Economics"),
            level: 100,
        },
        Student {
            name: String::from("Shania Bolade"),
            matric_number: String::from("CSC10328828"),
            department: String::from("Computer"),
            level: 200,
        },
        Student {
            name: String::from("Adekunle Gold"),
            matric_number: String::from("EEE11020202"),
            department: String::from("Electrical"),
            level: 200,
        },
        Student {
            name: String::from("Blanca Edemoh"),
            matric_number: String::from("MEE10202001"),
            department: String::from("Mechanical"),
            level: 100,
        },
    ];

    // Open a file for writing
    let mut file = File::create("students.txt")
        .expect("Unable to create file");

    // Read from the vector, display on screen, and write to file
    for student in &students {
        println!("Student Name: {}", student.name);
        println!("Matric. Number: {}", student.matric_number);
        println!("Department: {}", student.department);
        println!("Level: {}", student.level);

        writeln!(file, "Student Name: {}", student.name).unwrap();
        writeln!(file, "Matric. Number: {}", student.matric_number).unwrap();
        writeln!(file, "Department: {}", student.department).unwrap();
        writeln!(file, "Level: {}", student.level).unwrap();
        writeln!(file, "-").unwrap();
    }

    println!("Student records saved to students.txt");
}
