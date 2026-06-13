#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FState {
    pub expanded: u64,
    pub xored: u64,
    pub sboxed: u32,
    pub pboxed: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RoundState {
    pub l_in: u32,
    pub r_in: u32,
    pub subkey: u64,

    pub f_state: FState,

    pub l_out: u32,
    pub r_out: u32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IndexedRoundState {
    pub index: usize,
    pub state: RoundState,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DesState {
    pub input: u64,
    pub key: u64,

    pub ip_output: u64,

    pub subkeys: [u64; 16],
    pub rounds: [IndexedRoundState; 16],

    pub pre_output: u64,

    pub output: u64,
}
