// use crate::{
//     crypto::{mode::Mode, AES},
//     types,
// };

// use super::from_vec;

// #[test]
// fn test_aes_encrypt_ecb() {
//     let data: [u8; 16] = [
//         0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2,
//         0xe0, 0x37, 0x07, 0x34,
//     ];

//     let key: [u8; 16] = [
//         0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88,
//         0x09, 0xcf, 0x4f, 0x3c,
//     ];

//     let aes = AES::new(None, key, Mode::ECB);

//     let res = aes.encrypt(data.to_vec()).unwrap();

//     // println!("res: {:?}", types::u8_vec_to_hex_string(res));
//     println!("res: {:02x?}", res);
//     assert_eq!(
//         res,
//         [
//             // 0x39, 0x02, 0xdc, 0x19, //
//             // 0x25, 0xdc, 0x11, 0x6a, //
//             // 0x84, 0x09, 0x85, 0x0b, //
//             // 0x1d, 0xfb, 0x97, 0x32
//             0x39, 0x25, 0x84, 0x1d, //
//             0x02, 0xdc, 0x09, 0xfb, //
//             0xdc, 0x11, 0x85, 0x97, //
//             0x19, 0x6a, 0x0b, 0x32
//         ]
//     );
// }

// #[test]
// fn key_expansion() {
//     let key: [u8; 16] = [
//         0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88,
//         0x09, 0xcf, 0x4f, 0x3c,
//     ];

//     let aes = AES::new(None, key, Mode::ECB);

//     let ext_key = aes.key_expansion().unwrap();

//     assert_eq!(ext_key[10][12], 0xb6);
//     assert_eq!(ext_key[10][13], 0x63);
//     assert_eq!(ext_key[10][14], 0x0c);
//     assert_eq!(ext_key[10][15], 0xa6);
// }
