pub fn hex_to_base64(mut hex_string: String) -> String {
    if hex_string.len() % 2 == 1 {
        hex_string = format!("{}{}", hex_string, "0")
    };

    let mut base64_bytes_vec = Vec::new();
    for i in 0..(hex_string.len() / 2) {
        let mut hex_value = 0;
        hex_value += 16 * u8::from_str_radix(&hex_string[(2 * i)..(2 * i + 1)], 16).unwrap();
        hex_value += u8::from_str_radix(&hex_string[(2 * i + 1)..(2 * i + 2)], 16).unwrap();

        base64_bytes_vec.push(hex_value);
    }
    base64::encode(base64_bytes_vec)
}
