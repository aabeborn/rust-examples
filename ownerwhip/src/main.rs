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
    

    // Ways Variables and data interact: move
}
