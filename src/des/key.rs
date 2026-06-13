pub fn pc1(input: u64) -> u64 {
    static PC1: [usize; 56] = [
        57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26, 18, 10, 2, 59, 51, 43, 35, 27, 19, 11, 3,
        60, 52, 44, 36, 63, 55, 47, 39, 31, 23, 15, 7, 62, 54, 46, 38, 30, 22, 14, 6, 61, 53, 45,
        37, 29, 21, 13, 5, 28, 20, 12, 4,
    ];

    let mut output: u64 = 0;

    for i in 0..56 {
        let src = PC1[i] - 1;
        let bit = (input >> (63 - src)) & 1;
        output |= bit << (55 - i);
    }

    output
}

pub fn pc2(input: u64) -> u64 {
    static PC2: [usize; 48] = [
        14, 17, 11, 24, 1, 5, 3, 28, 15, 6, 21, 10, 23, 19, 12, 4, 26, 8, 16, 7, 27, 20, 13, 2, 41,
        52, 31, 37, 47, 55, 30, 40, 51, 45, 33, 48, 44, 49, 39, 56, 34, 53, 46, 42, 50, 36, 29, 32,
    ];

    let mut output: u64 = 0;

    for i in 0..48 {
        let src = PC2[i] - 1;
        let bit = (input >> (55 - src)) & 1;
        output |= bit << (47 - i);
    }

    output
}

fn left_shift28(x: u64, n: u8) -> u64 {
    let x = x & 0x0FFFFFFF;
    let y = ((x << n) | (x >> (28 - n))) & 0x0FFFFFFF;

    y
}

pub fn key_schedule(key: u64) -> [u64; 16] {
    static SHIFTS: [u8; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];

    let mut subkeys = [0u64; 16];

    let key56 = pc1(key);

    let mut c = (key56 >> 28) & 0x0FFFFFFF;
    let mut d = key56 & 0x0FFFFFFF;

    for i in 0..16 {
        let shift = SHIFTS[i];

        c = left_shift28(c, shift);
        d = left_shift28(d, shift);

        let cd = (c << 28) | d;

        subkeys[i] = pc2(cd);
    }

    subkeys
}
