
use std::io;

fn main() {
    let professions = vec![
        "Office Admin",
        "Academic",
        "Lawyer",
        "Teacher",
    ];

    let levels = vec![
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "ELI 8-10",
        "EL2 10-13",
        "SES",
    ];

    let positions = vec![
        vec!["Intern", "-", "Paralegal", "Placement"],
        vec![
            "Administrator",
            "Research Assistant",
            "Junior Associate",
            "Classroom Teacher",
        ],
        vec![
            "Senior Administrator",
            "PhD Candidate",
            "Associate",
            "Senior Teacher",
        ],
        vec![
            "Office Manager",
            "Post-Doc Researcher",
            "Senior Associate 1-2",
            "Leading Teacher",
        ],
        vec![
            "Director",
            "Senior Lecturer",
            "Senior Associate 3-4",
            "Deputy Principal",
        ],
        vec!["CEO", "Dean", "Partner", "Principal"],
    ];

    println!("PUBLIC SERVICE APS LEVEL CHECKER");
    println!("Available professions:");
    
    for profession in &professions {
        println!("{}", profession);
    }

    println!("Enter your profession:");
    let mut profession = String::new();
    io::stdin().read_line(&mut profession).unwrap();

    let profession = profession.trim().to_lowercase();

    let mut profession_index = None;

    for i in 0..professions.len() {
        if professions[i].to_lowercase() == profession {
            profession_index = Some(i);
        }
    }

    match profession_index {
        Some(index) => {
            println!("Enter years of experience:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let experience: u32 = match input.trim().parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("Invalid experience entered.");
                    return;
                }
            };

            let level_index = if experience <= 2 && experience >= 1 {
                0
            } else if experience <= 4 && experience >= 3 {
                1
            } else if experience <= 7 && experience >= 5 {
                2
            } else if experience <= 9 && experience >= 8 {
                3
            } else if experience <= 13 && experience >= 10 {
                4
            } else {
                5
            };

            if experience == 0 || experience > 13 {
                println!("Experience is outside the defined ranges.");
            } else {
                println!("\nRESULT");
                println!("Profession: {}", professions[index]);
                println!("Years of experience: {}", experience);
                println!("Staff position: {}", positions[level_index][index]);
                println!("APS Level: {}", levels[level_index]);
            }
        }

        None => {
            println!("Invalid profession entered.");
        }
    }
}
