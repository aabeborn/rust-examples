fn main() {
    // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. 
    // A slice is a kind of reference, so it does not have ownership.

    let mut s = String::from("hello world!");
    let word = first_word(&s);

    s.clear();

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // so we can use string slices!
    // String slice is a reference to a part of a string.
    
    let s = String::from("hello world!");

    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[..2]; // same as 0..2

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

    let slice = &s[..];

    // The type of s here is &str: it’s a slice pointing to that specific point of the binary. 
    // This is also why string literals are immutable; &str is an immutable reference.
    let s = "Hello, world!";

    // We can use slices also with other ty
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3])
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// better using &str because in this way we can use both strings and string literals (slices of a string)
fn first_word_slicing(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}