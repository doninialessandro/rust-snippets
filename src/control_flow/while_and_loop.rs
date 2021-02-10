pub fn function() {
    println!("--- WHILE AND LOOP ---");
    let mut x = 1;
    while x < 1000 {
        x *= 2;
        if x == 64 {
            continue; // the continue instruction go directly to the next step withou executing the remainin code
        }
        println!("x = {}", x);
    }

    // while true
    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {}", y);

        // stop at 2^10
        if y == 1 << 10 {
            break;
        }
    }
}
