fn main(){
   
    let  name = String::from("mishal");

    let  byte_arr = name.as_bytes();

    let mut  new_reversed_string  = String::new();

    let mut i = byte_arr.len();


    while i >0 {
        i -=1;
        new_reversed_string.push(byte_arr[i] as char);
    }

    println!("{}",new_reversed_string );

    
}