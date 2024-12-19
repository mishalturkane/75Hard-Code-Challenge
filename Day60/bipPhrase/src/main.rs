use bip39::{Mnemonic, Language};

fn main() {
    // Generate a 12-word mnemonic phrase
    let mnemonic = Mnemonic::generate_in(Language::English, 12).unwrap();

    // Print the mnemonic phrase
    println!("Generated Mnemonic: {}", mnemonic.phrase());
}
