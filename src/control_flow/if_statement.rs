use std::io;

pub fn function() {
    println!("--- IF STATEMENT ---");
    println!("How many degrees are there outside?");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("failed to read from stdin");

    let trimmed = temp.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => {
            if i > 30 {
                println!("Really hot outside!")
            } else if i < 10 {
                println!("Really cold outside")
            } else {
                println!("The temperature outside is good")
            }
        }
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
}
