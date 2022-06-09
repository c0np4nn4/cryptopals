mod hex_to_base64;

use hex_to_base64::hex_to_base64;

fn main() {
    let hex_string:String = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("Input: {}", hex_string);
    println!("Result: {}", hex_to_base64(hex_string));

    let hex_string:String = String::from("a49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("Input: {}", hex_string);
    println!("Result: {}", hex_to_base64(hex_string));

    let hex_string:String = String::from("aa49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    println!("Input: {}", hex_string);
    println!("Result: {}", hex_to_base64(hex_string));
}
