fn main() {
    println!("Hello, world!");

    another_function();
    parametric_function(5);
    print_labeled_measuerments(5, 'h');

    // statements: are instructions that perform some action and do not return a value. 
    // expressions: evaluate to a resulting value
    // this is a function
    let x = {
        let y = 3;
        // if no ; it is an expression => returns y + 1
        // if there is a ; at the end of the function, it will be a statement => no return value
        y + 1
    };
    println!("Example of expression: {}", x);

    let x = five();

    println!("Function five returns: {}", x);

    let x = plus_one(7);

    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

fn parametric_function(x: i32) {
    println!("The value of the param is: {}", x);
}

fn print_labeled_measuerments(value: i32, label: char ) {
    println!("The measurement is: {}{}", value, label);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}