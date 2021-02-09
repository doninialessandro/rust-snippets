fn scope_and_shadowing() {
    let a = 123;
    let c = 5667;
    {
        let a = 65; // a has different values. The inner variable kind of shadows the outer variable.
        println!("inside, a = {}. Inside a is a different variable.", a);
        let b = 465;
        println!("inside, b = {}", b);
        println!("inside, c = {}", c);
    }
    println!("outside, a = {}", a);
    println!("outside, c = {}", c);
    // println!("outside, b = {}", b); b is not available outside the scope in which it is declared
}

pub fn function() {
    println!("--- SCOPE AND SHADOWING ---");
    scope_and_shadowing()
    // a only exist in the scope of the function
}

// fn function2() {
// a is not available here
// }
