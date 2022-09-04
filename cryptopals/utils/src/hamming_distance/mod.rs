#[cfg(test)]
mod test;

use crate::BoxedError;

pub fn get_hamming_distance(l: &[u8], r: &[u8]) -> Result<u64, BoxedError> {
    if l.len() != r.len() {
        return Err(format!(
            "length is different, {} and {}",
            l.len(),
            r.len()
        )
        .into());
    }

    let mut res = Vec::<u8>::new();

    for idx in 0..l.len() {
        res.push(l[idx] ^ r[idx]);
    }

    let mut distance: u64 = 0;
    res.into_iter().for_each(|b| {
        distance += b.count_ones() as u64;
    });

    Ok(distance)
}
