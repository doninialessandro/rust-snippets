pub fn function() {
    println!("--- OPERATORS ---");
    println!("- ARITHMETIC");
    let mut a = 2 + 3 * 4;
    println!("My first operation (2 + 3 * 4) give me: {}", a);

    a += 2; // a = a + 2
            // we have also -= *= /= %=

    println!("Adding 2 to a we obtain: {}", a);
    println!("The remainder of {} / {} is: {}", a, 3, (a % 3));

    let a_cubed = i32::pow(a, 3);
    println!("The cube of {} is: {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    println!("- BITWISE");
    // only available for integers
    // | OR, & AND, ^ XOR, ! NEGATION
    let c = 1 | 2; // 01 or 10 = 11 == 3_10
    println!("1|2 = {} ", c);

    // shift operator
    let two_to_10 = 1 << 10; // >>
    println!("2^10 = {} ", two_to_10);

    println!("- LOGICAL");
    let pi_less_4 = std::f64::consts::PI < 4.0;
    println!(
        "{} is less then {} ? {} ",
        std::f64::consts::PI,
        4,
        pi_less_4
    );
    // > <= => ==
}
