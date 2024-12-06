use std::collections::HashMap;

fn main() {
    // Create a HashMap to store student enrollment numbers and their names
    let mut student_map: HashMap<i32, String> = HashMap::new();

    // Adding 10 students with random enrollment numbers and names
    student_map.insert(1001, String::from("John Doe"));
    student_map.insert(1002, String::from("Jane Smith"));
    student_map.insert(1003, String::from("Emily Johnson"));
    student_map.insert(1004, String::from("Michael Brown"));
    student_map.insert(1005, String::from("Sarah Williams"));
    student_map.insert(1006, String::from("David Jones"));
    student_map.insert(1007, String::from("Laura Garcia"));
    student_map.insert(1008, String::from("Chris Martinez"));
    student_map.insert(1009, String::from("Anna Lee"));
    student_map.insert(1010, String::from("James White"));

    // Print the HashMap to see the contents
    for (enrollment, name) in &student_map {
        println!("Enrollment Number: {}, Name: {}", enrollment, name);
    }
}
