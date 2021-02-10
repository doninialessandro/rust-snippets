use std::io;

pub fn function() {
    println!("--- MATCH ---");

    println!("Please, insert a country code:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");

    let trimmed = input.trim();
    match trimmed.parse::<u32>() {
        Ok(country_code) => {
            let country = match country_code {
                44 => "UK",
                46 => "Sweden",
                1..=1000 => "unknown", // inclusive range ... is deprecated
                _ => "invalid",        // try commenting this out - must cover all cases!
            };
            println!("The country with code {} is {}", country_code, country);
        }
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
}
