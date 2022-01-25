fn main() {
    // instead of passing a value with its ownership, we can pass onlt the value
    // Explained the problem in the ownership project

    // a Reference is like a pointer in that i;s an address we can follow to access data stored at that address
    // that is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value.

    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of {} is {}", s1, len);

    // the action of reference creation is called Borrowing
    // obviously we can't edit a reference value, because it doesn't own the value.

    // reference are immutable by default!

    // MUTABLE REFERENCES
    let mut s= String::from("Hello");
    change(&mut s);

    // !IMPORTANT Mutable references have one big restriction: you can have only one mutable reference to a particular piece of data at a time
    // The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion
    // this is pretty useful because it allows to avoid data race problems
    // data race a race condition that occurs:
    // - Two or more pointers access the same data at the same time;
    // - At least one of the pointers is being used to write to the data
    // - There's  no mechanism beign used to synchronize access to the data

    // obviosuly we can create a new scope to avoid this
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    
    // we cannot also mix mutable and immutable reference; we can declare multiple immutable references at the same timee anyway 
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // The scopes of the immutable references r1 and r2 end after the println! where they are last used, which is before the mutable reference r3 is created. These scopes don’t overlap, so this code is allowed. 

    // DANGLING REFERENCES
    // In languages with pointers, 
    // it’s easy to erroneously create a dangling pointer--a pointer that references a location in memory that may have been given to someone else--
    // by freeing some memory while preserving a pointer to that memory

    //In Rust, by contrast, the compiler guarantees that references will never be dangling references: 
    // if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.
    // For example returing a reference from a function generates an error.

    // RULES
    // -  At any given time, you can have either one mutable reference or any number of immutable references.
    // -  References must always be valid. 
}

fn calculate_length(s: &String) -> usize {
     // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.


fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}