use crate::des::key::key_schedule;
use crate::des::permute::{fp, ip};
use crate::des::round::round;
use crate::des::states::{DesState, Indexed, RoundState};

pub fn des_encrypt_decrypt(input: u64, key: u64, decrypt_mode: bool) -> (u64, DesState) {
    let mut subkeys = key_schedule(key);
    if decrypt_mode {
        subkeys.reverse();
    }
    let subkeys = subkeys;

    let iped = ip(input);

    let mut l = (iped >> 32) as u32;
    let mut r = iped as u32;

    let mut indexed_rounds: [Indexed<RoundState>; 16] =
        std::array::from_fn(|_| Indexed::<RoundState>::default());
    let mut indexed_subkeys: [Indexed<u64>; 16] =
        std::array::from_fn(|_| Indexed::<u64>::default());

    for i in 0..16 {
        let subkey = subkeys[i];
        let (nl, nr, r_state) = round(l, r, subkey);

        indexed_rounds[i] = Indexed::<RoundState> {
            index: i + 1,
            state: r_state,
        };

        indexed_subkeys[i] = Indexed::<u64> {
            index: i + 1,
            state: subkey,
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
        subkeys: indexed_subkeys,
        rounds: indexed_rounds,
        pre_output: final_swaped,
        output: output,
    };

    (output, state)
}
