fn main() {
    let string = "hello world";
    let substring = "world";

    if string.contains(substring) {
        println!("The substring '{}' exists in '{}'.", substring, string);
    } else {
        println!("The substring '{}' does not exist in '{}'.", substring, string);
    }
}
