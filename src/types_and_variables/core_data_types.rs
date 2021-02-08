use std::mem;

pub fn function() {
    println!("--- CORE DATA TYPES ---");
    println!("- NUMBERS");
    // let binding
    let a: u8 = 4; // u = unsigned, 8 bits, 0 - 255
    println!("a: My first u8 number, called a, is: {}", a); // a is immutable, cannot be change

    // u = unsinged, 0 ... 2^N-1
    // i = signed, -2^(N-1) ... 2^(N-1)-1 (due to 0 value)
    let mut b: i8 = 8;
    println!("b: My first i8 number, called b, is: {}", b);
    b = -4;
    println!("b: ... but now b is mutate, and its value is: {}", b);

    let c = 120000; // i32 = 32bits = 4 bytes
    println!(
        "c: Mmm let Rust calculate how many bytes c (with value {}) has .... c takes up {} bytes",
        c,
        mem::size_of_val(&c)
    );

    // u8, u16, u32, u64, i8, i16 ..

    // usize isize
    let z: isize = 123;
    let z_size: usize = mem::size_of_val(&z);
    println!(
        "z: z = {}, takes up {} bytes, {}-bit OS",
        z,
        z_size,
        z_size * 8
    );

    println!("- FLOATING POINT NUMBERS");
    // f32 f64, signed! they can be negative and positive
    // f64 is the default
    let f: f32 = 4.4;
    println!(
        "f: My first floating point, called f, is {}, and takes up {} bytes",
        f,
        mem::size_of_val(&f)
    );

    let g: f64 = 4.444444;
    println!(
        "g: My second floating point, called f, is {}, and takes up {} bytes",
        g,
        mem::size_of_val(&g)
    );

    println!("- CHARACTERS");
    let d: char = 'x'; // 32 bits
    println!(
        "d: My first char, called d, is {}, and takes up {} bytes",
        d,
        mem::size_of_val(&d)
    );

    println!("- BOOLEAN");
    let i: bool = true;
    println!(
        "i: My first boolean, called i, is {}, and takes up {} bytes",
        i,
        mem::size_of_val(&i)
    );
}
