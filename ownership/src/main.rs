fn main() {
    // Some languages have garbage collection that constantly looks for no-longer used memory as the program runs; 
    // in other languages, the programmer must explicitly allocate and free the memory. 
    // Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. 
    // If any of the rules are violated, the program won’t compile. 
    // None of the features of ownership will slow down your program while it’s running.
    
    // for more info https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

    // two types of "memory": stack and heap
    // stack uses lifo to store data, data must have a known and fixed size
    // heap is randomic => when a space big enough has found store the variable and return a pointer
    // access data from the heap is way slower (it has to follow a pointer) also storing is slower (it needs to search for the free space)
    
    // calling a function params and local variables are pushed to the stack when the function ends the values are removed

    // main purpose of the ownership is to correctly manage the heap which is the most critic part of the memory in terms of performances

    // Rules:
    // - Each value has a variable that's called its owner.
    // - There can be one owner at a time.
    // - When the owner goes out of scope, the value will be dropped.


    println!("Hello, world!");

    // variable scope

    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
        println!("{}", s);
        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    // Complex data types 
    
    // to store data with an unknow size the heap must be used
    // an example is the type String
    // create the string starting from a string literal
    let s = String::from("hello");
    
    // The double colon :: operator allows us to namespace this particular from function under the String type rather than using some sort of name like string_from
    let mut s = String::from("hello");
    
    s.push_str(", world!"); // appends a literal to a String

    println!("{}", s);

    // why we can mutate the string?
    // In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable.
    // This is why string literals are fast and efficient. 
    // But these properties only come from the string literal’s immutability. 
    // Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.
    // With the String type, in order to support a mutable, growable piece of text, 
    // we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. 
    // This means:
    // - The memory must be requested from the memory allocator at runtime.
    // - We need a way of returning this memory to the allocator when we're done with our String
    // That first part is done by us: when we call String::from, its implementation requests the memory it needs. 
    // This is pretty much universal in programming languages.
    // The memory is automatically returned (cleaned) once the variable that owns it goes out of scope. 
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
    // When the variable goes out of scope Rust calls a special function called drop and it’s where the author of String can put the code to return the memory. 
    // Rust calls drop automatically at the closing curly bracket.
    

    // Ways Variables and data interact: MOVE

    // multiple data can interact with the same data in different ways
    let x = 5;
    let y = x;

    // pushed into the stack (basic types) so created a copy

    let s1 = String::from("Hello");
    let s2 = s1;

    // this is different
    // String is composed by three parts:
    // - a pointer to the memomry which holds the value,
    // - length
    // - capacity

    // this is stored inside the stack, the real value is stored inside the heap

    // length is how much memory in bytes the string is using.
    // capacity is the amount of memory which the allocator assign to string

    // when we assign s1 to s2 the data inside the stack is copied, not the real value

    // this causes a potential problem: when rust calls the drop function when the variables goes out of scope
    // they try to free the same memory. This is known as a double free error and is one of the memory safety bugs mentioned
    // freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilites

    // to ensure memory safety after assinging s1 to s2, s1 is no longer valid and available. Compilation error!
    
    // this conecpt of copying the pointer length and capacity in other langauges is called shallow copy.
    // In rust due to the fact that the copied variable has been invalidated, is called MOVE.
    
    // Design choice.... Rust never automatically creates deep copies of the data. 

    // Ways Variables and data interact: CLONE

    // to create a deep copy of a variable (not only the stack but also the heap) we can use clone.

    let s1 = String::from("Hello");
    let s2 = s1.clone();

    // this could be expensive!

    // Stack-Only data: Copy

    // so the move doesn't completely apply to the basic types which are completely stored inside the stack
    // there is no difference between the shallow or deep copy, because the size is fixed

    // Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack like integers are
    // If a type implements the Copy trait, a variable is still valid after assignment to another variable

    // usually only scalar types could implement copy (they don't have any drop function) and nothing that requires the allocation can implement Copy.
    // examples:
    // - All integers, such as u32;
    // - Boolean
    // - All floating point, such as f64
    // - Char type
    // - Tuples, if they only contains type that implements Copy.


    // Ownership and functions

    // The semantics for passing a value to a function are similar to those for assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does
    let s = String::from("Hello");      // s comes into scope
    takes_ownership(s);                 // s's value moves into the function...
                                        // ... and so is no longer valid here
    let x = 5;                          // x coming into scope
    makes_copy(x);                      // x would move into the function
                                        // butt i32 is Copy, so it's okay to still use x afterwrd
    // when going out from main, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens


    // Return values and scope
    // returning values can also transfer ownership.

    let s1 = gives_ownership();         // gives_ownership moves its return value into s1

    let s2 = String::from("Hello");     // s2 come into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into 
                                        // takes_and_gives_back, which also moves its return value into s3
    // when going out from main, s3 goes out of scoped and is dropped, s2 was moved, so nothing happens, s1 goes out of scope and is dropped.

    // return multiple values
    let s1 = String::from("Hello");
    
    let (s2, len) = calculate_length(s1);

    println!("The length of {} is {}", s2, len);


    // but this could be really tedious, every time we need to pass and return the passed values to not lose them
    // fortunately Rust aas a feature called references for passing values without passing ownership
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String { // will move its return value into the function that calls it
    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

// this function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into space

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // returns the length of a String

    (s, length)
}