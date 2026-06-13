use crate::des::key::key_schedule;
use crate::des::permute::{fp, ip};
use crate::des::round::round;
use crate::des::states::{DesState, IndexedRoundState};

pub fn des_encrypt_decrypt(input: u64, key: u64, decrypt_mode: bool) -> (u64, DesState) {
    let mut subkeys = key_schedule(key);
    if decrypt_mode {
        subkeys.reverse();
    }

    let iped = ip(input);

    let mut l = (iped >> 32) as u32;
    let mut r = iped as u32;

    let mut rounds: [IndexedRoundState; 16] = std::array::from_fn(|_| IndexedRoundState::default());

    for i in 0..16 {
        let subkey = subkeys[i];
        let (nl, nr, r_state) = round(l, r, subkey);

        rounds[i] = IndexedRoundState {
            index: i + 1,
            state: r_state,
        };

        l = nl;
        r = nr;
    }

    let final_swaped = ((r as u64) << 32) | (l as u64);
    let output = fp(final_swaped);

    let state = DesState {
        input: input,
        key: key,
        ip_output: iped,
        subkeys: subkeys,
        rounds: rounds,
        pre_output: final_swaped,
        output: output,
    };

    (output, state)
}
