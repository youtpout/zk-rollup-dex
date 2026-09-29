pub struct Token {
    chain_id: u32,
    is_native: bool,
    is_pair: bool,
    address: [u8; 32],
}
