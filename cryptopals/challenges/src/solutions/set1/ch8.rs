use std::fs;

#[test]
fn chal_8() {
    let data: Vec<String> = fs::read_to_string("../../data/8.txt")
        .unwrap()
        .split_ascii_whitespace()
        .map(|ct| ct.to_string())
        .collect();

    let mut result_table: Vec<Vec<(usize, usize)>> = vec![vec![]];

    for candidate_idx in 0..data.len() {
        let candidate_data = data[candidate_idx].clone();

        let mut result_at_candidate_idx: Vec<(usize, usize)> = vec![];
        for i in (0..candidate_data.len()).step_by(16) {
            for j in (i + 16..candidate_data.len()).step_by(16) {
                let chunk_1 = candidate_data[i..i + 16].as_bytes();
                let chunk_2 = candidate_data[j..j + 16].as_bytes();

                if chunk_1.eq(chunk_2) {
                    result_at_candidate_idx.push((i, j));
                }
            }
        }
        result_table.push(result_at_candidate_idx);
    }

    for (idx, result) in result_table.iter().enumerate() {
        // println!("\nidx: {:?}", idx);
        if !result.is_empty() {
            println!("idx: {:?}, detected chunks idices: \n{:?}", idx, result);
        }
    }

    // let mut score_table = Vec::new();

    // for candidate_idx in 0..data.len() {
    //     let mut score = 0;

    //     let candidate_data = data[candidate_idx].clone();

    //     for i in (0..candidate_data.len()).step_by(16) {
    //         println!("");
    //         for j in (i + 16..candidate_data.len()).step_by(16) {
    //             score += hamming_distance::get_hamming_distance(
    //                 candidate_data[i..i + 16].as_bytes(),
    //                 candidate_data[j..j + 16].as_bytes(),
    //             )
    //             .unwrap();
    //         }
    //     }

    //     score_table.push(score);
    // }

    // let mut score_table_clone = score_table.clone();

    // score_table_clone.sort();

    // let (min_value, min_value_idx) = {
    //     let min_value = score_table_clone[0];

    //     let min_value_idx = score_table
    //         .iter()
    //         .position(|score| *score == min_value)
    //         .unwrap();

    //     (min_value, min_value_idx)
    // };

    // println!("value: {:?}, idx: {:?}", min_value, min_value_idx);

    // println!("Detected aes encrypted text: {:?}", data[min_value_idx]);
}
