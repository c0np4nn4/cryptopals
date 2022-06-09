pub fn hex_string_to_vec(mut hex_string: String) -> Vec<u8> {
    if hex_string.len() % 2 == 1 {
        hex_string = format!("{}{}", "0", hex_string);
    };

    let mut base64_bytes_vec = Vec::new();
    for i in 0..(hex_string.len() / 2) {
        let mut hex_value = 0;
        hex_value += 16 * u8::from_str_radix(&hex_string[(2 * i)..(2 * i + 1)], 16).unwrap();
        hex_value += u8::from_str_radix(&hex_string[(2 * i + 1)..(2 * i + 2)], 16).unwrap();

        base64_bytes_vec.push(hex_value);
    }

    base64_bytes_vec
}

pub fn fixed_xor(vec_1: Vec<u8>, vec_2: Vec<u8>) -> String {
    let mut result = String::new();

    for it in vec_1.iter().zip(vec_2.iter()) {
        let (value_1, value_2) = it;

        let value = format!("{:x}", value_1 ^ value_2);
        result = format!("{}{}", result, value);
    }

    result
}

fn main() {
    let hex_string_1 = String::from("1c0111001f010100061a024b53535009181c");
    let hex_string_2 = String::from("686974207468652062756c6c277320657965");

    let hex_vec_1 = hex_string_to_vec(hex_string_1);
    let hex_vec_2 = hex_string_to_vec(hex_string_2);

    let result = fixed_xor(hex_vec_1, hex_vec_2);
    println!("Result: {}", result);
}
