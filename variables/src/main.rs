fn main() {
    // variable are by default immutable
    let x = 5;
    println!("The value of x is: {}", x);
    // this will throw a compilation error
    // x = 5;
    // mutable variables declaration
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // const differ from let because type annotation is needed
    // can be declared also in global scope
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of const is: {}", THREE_HOURS_IN_SECONDS);
    // shadowing
    // you can declare a new variable with the same name as a previous variable. Rustaceans say that the first variable is shadowed by the second, which means that the second variable’s value is what the program sees when the variable is used
    // really useful to change variable type  
    let x = 5;
    let x = x + 1;
    // inner scope
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    // back to normal scope
    println!("The value of x is: {}", x);

}
