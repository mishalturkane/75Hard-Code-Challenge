fn main() {
    let name = String::from("mishal");
    let bytes = name.as_bytes(); // Convert the string to a byte array
    let mut reversed_name = String::new(); // Create an empty String to store the reversed name

    // Iterate through the byte array from the last index to the first
    let mut i = bytes.len();
    while i > 0 {
        i -= 1;
        // Push each character from the end of the array to the new string
        reversed_name.push(bytes[i] as char);
    }

    println!("Original: {}", name);
    println!("Reversed: {}", reversed_name);
}
