const MEANING_OF_LIFE: u8 = 43; // no fixed adress
static Z: i32 = 123; // fixed adress
static mut ZU: i32 = 123;

pub fn function() {
    println!("--- CONSTANTS ---");
    println!("const: {}", MEANING_OF_LIFE);
    println!("static: {}", Z);
    unsafe {
        ZU = 4;
        println!("static (unsafe): {}", ZU);
    }
}
