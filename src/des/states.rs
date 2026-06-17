#[derive(Clone, Debug, Default)]
pub struct FState {
    pub expanded: u64,
    pub xored: u64,
    pub sboxed: u32,
    pub pboxed: u32,
}

#[derive(Clone, Debug, Default)]
pub struct RoundState {
    pub l_in: u32,
    pub r_in: u32,
    pub subkey: u64,

    pub f_state: FState,

    pub l_out: u32,
    pub r_out: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Indexed<T> {
    pub index: usize,
    pub state: T,
}

#[derive(Clone, Debug, Default)]
pub struct DesState {
    pub input: u64,
    pub key: u64,

    pub ip_output: u64,

    pub subkeys: [Indexed<u64>; 16],
    pub rounds: [Indexed<RoundState>; 16],

    pub pre_output: u64,

    pub output: u64,
}
