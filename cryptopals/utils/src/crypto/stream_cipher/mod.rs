use super::PRNG;

// 32-bit
pub fn encrypt<T>(prng: &mut T, pt: Vec<u8>) -> Vec<u8> where T: PRNG{
    let mut res = Vec::<u8>::default();

    let mut pt = pt;

    while pt.len() % 4 != 0 {
        pt.push(0u8);
    }

    for i in (0..pt.len()).step_by(4) {
        let mut pt_hex: u32 = 0;
        // println!("pt_0: {:?}", (pt[i + 0] as u32) << 24);
        // println!("pt_1: {:?}", (pt[i + 1] as u32) << 16);
        // println!("pt_2: {:?}", (pt[i + 2] as u32) << 8);
        // println!("pt_3: {:?}", pt[i + 3]);

        pt_hex += (pt[i + 0] as u32) << 24;
        pt_hex += (pt[i + 1] as u32) << 16;
        pt_hex += (pt[i + 2] as u32) << 8;
        pt_hex += (pt[i + 3] as u32) << 0;

        // println!("pt_4byte: {:?}", pt_hex);

        let ct_hex = pt_hex ^ prng.extract();

        res.push(((ct_hex & 0xff_00_00_00) >> 24) as u8);
        res.push(((ct_hex & 0x00_ff_00_00) >> 16) as u8);
        res.push(((ct_hex & 0x00_00_ff_00) >> 8)  as u8);
        res.push(((ct_hex & 0x00_00_00_ff) >> 0)  as u8);
    }

    res
}

pub fn decrypt<T>(prng: &mut T, ct: Vec<u8>) -> Vec<u8> where T: PRNG{
    let mut res = Vec::<u8>::default();

    let mut ct = ct;
    
    while ct.len() % 4 != 0 {
        ct.push(0u8);
    }

    for i in (0..ct.len()).step_by(4) {
        let mut ct_hex: u32 = 0;
        // println!("ct_0: {:?}", (ct[i + 0] as u32) << 24);
        // println!("ct_1: {:?}", (ct[i + 1] as u32) << 16);
        // println!("ct_2: {:?}", (ct[i + 2] as u32) << 8);
        // println!("ct_3: {:?}", ct[i + 3]);

        ct_hex += (ct[i + 0] as u32) << 24;
        ct_hex += (ct[i + 1] as u32) << 16;
        ct_hex += (ct[i + 2] as u32) << 8;
        ct_hex += (ct[i + 3] as u32) << 0;

        // println!("pt_4byte: {:?}", pt_hex);

        let pt_hex = ct_hex ^ prng.extract();

        res.push(((pt_hex & 0xff_00_00_00) >> 24) as u8);
        res.push(((pt_hex & 0x00_ff_00_00) >> 16) as u8);
        res.push(((pt_hex & 0x00_00_ff_00) >> 8) as u8);
        res.push(((pt_hex & 0x00_00_00_ff) >> 0) as u8);
    }

    res
}
