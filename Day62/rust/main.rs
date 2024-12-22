struct Person {
    name: String,
    age: u8,
    is_student: bool,
}

fn main() {
 
    let person = Person {
        name: String::from("Alice"),
        age: 25,
        is_student: true,
    };

    
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Is Student: {}", person.is_student);

    let mut person2 = Person {
        name: String::from("Bob"),
        age: 30,
        is_student: false,
    };
    person2.age = 31; 
    println!("Updated Age: {}", person2.age);
}
