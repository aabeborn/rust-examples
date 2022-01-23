fn main() {
    // ------- CONDITIONS --------
   
    let number = 5;
    // Condition have to be bool
    // you can't do if number, something normal in js
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    let number = 6;
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3  == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible neither by 4, 3 and 2 ");
    }

    let condition = true;
    // Obviously  we can't use value of different type
    let number = if condition { 5 } else { 6 };

    println!("The value of the number is: {}", number);

    // -------- LOOPS -----------
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1
    } 
    println!("End count = {}", count);

    let mut counter = 0;
    
    let result = loop {
        
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;
    
    while number != 3 {
        println!("{}!", number);

        number -= 1;
    };
    
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        
        index += 1;
    }

    for element in a {
        println!("The value is: {}", element);
    }

    // use range to loops between two numbers 
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
}