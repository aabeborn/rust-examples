fn main() {
    // scalar types
    //      - integers
    //          signed       unsigned
    //          * i8         u8
    //          * i16        u16
    //          * i32        u32
    //          * i64        u64
    //          * i128       u128   
    //          * isize      usize
    //          literals
    //          * Decimal 98_222
    //          * Hex 0Xff
    //          * octal 0o77
    //          * binary 0b11111_0000
    //          * Byte (u8 only) b'A'
    //          overflow
    //          - Wrap in all modes with the wrapping_* methods, such as wrapping_add
    //          - Return the None value if there is overflow with the checked_* methods
    //          - Return the value and a boolean indicating whether there was overflow with the overflowing_* methods
    //          - Saturate at the value’s minimum or maximum values with saturating_* methods
    //      - floating-pointer numbers
    //          * f64 (default)
    //          * f32
     // addition
     let sum = 5 + 10;

     // subtraction
     let difference = 95.5 - 4.3;
 
     // multiplication
     let product = 4 * 30;
 
     // division
     let quotient = 56.7 / 32.2;
     let floored = 2 / 3; // Results in 0
 
     // remainder
     let remainder = 43 % 5;

     println!("{} {} {} {} {} {}", sum, difference, product, quotient, floored, remainder);

    //      - bool
    let f: bool = false; // with explicit type annotation
    println!("{}", f);
    //      - char
    //          4 bytes => unicode char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{} {} {}", c, z, heart_eyed_cat);
    
    // compound types
    //      - tuples
    //          fixed length
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //              tup binds the entire tuple
    //              destructure to obtain indvidual values
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);
    //              or as always with a .
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;
    println!("{}, {}, {}", five_hundred, six_point_four, one);
    //              The tuple without any values, (), is a special type that has only one value, also written (). The type is called the unit type and the value is called the unit value
    //      - array
    //          fixed length
    let a = [1, 2, 3, 4, 5];
    println!("{}", a[1]);
    //              Arrays are useful when you want your data allocated on the stack rather than the heap               
    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", a[1]);
    let a = [3; 5]; // array of 5 elements with value 3
    let first = a[0];
    let second = a[1];
    println!("{} {} {} {} ", months[11], a[1], first, second);
}   
