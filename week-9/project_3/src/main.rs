use std::fs::File;
use std::io::Write;

fn main() {
    // Dataset 1: Commissioners
    let commissioners = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiye",
    ];

    // Dataset 2: Ministries
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    // Dataset 3: Geopolitical zones
    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    // Create the output file
    let mut file = File::create("efcc_merged.txt")
        .expect("Unable to create file");

    // Merge the datasets by index
    for i in 0..commissioners.len() {
        let record = format!(
            "{}, {}, {}",
            commissioners[i],
            ministries[i],
            zones[i]
        );

        // Display the merged record
        println!("{}", record);

        // Write the merged record to the file
        writeln!(file, "{}", record).unwrap();
    }

    println!("Merged records saved to efcc_merged.txt");
}

