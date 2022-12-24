use std::time::{SystemTime, UNIX_EPOCH};

use utils::{crypto::mt19937::MT19937, attack::mt19937_untempering::untemper};

fn get_next_state(rng: &mut MT19937) -> Vec<u32> {
    let mut res = vec![];

    for _ in 0..624 {
        let tmp = rng.extract();

        res.push(tmp);
    }

    res
}


#[test]
fn chal_23() {
    let seed: u32 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;

    let mut my_rng = MT19937::new();

    my_rng.init(seed);

    let mut state: Vec<u32>;

    // twisted count, original: 1
    state = get_next_state(&mut my_rng);

    for i in 0..624 {
        state[i] = untemper(state[i]);
    }

    let mut cloned_rng = MT19937::new_with_state(state.as_slice());

    // twisted count, original: 2, cloned: 0
    get_next_state(&mut my_rng);

    // twisted count, original: 2, cloned: 1
    get_next_state(&mut cloned_rng);

    // twisted count, original: 2, cloned: 2
    get_next_state(&mut cloned_rng);

    for _ in 0..624 {
        let val_1 = my_rng.extract();

        let val_2 = cloned_rng.extract();

        assert_eq!(val_1, val_2);
    }
}
