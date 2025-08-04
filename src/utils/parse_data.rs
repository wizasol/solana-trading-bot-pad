
use std::str::from_utf8;

use solana_sdk::pubkey::Pubkey;

/// Reads a little-endian u32 from the buffer starting at offset.
pub fn read_u64_le(data: &[u8], offset: &mut usize) -> u64 {
    let val = u64::from_le_bytes(data[*offset..*offset + 8].try_into().unwrap());
    *offset += 8;
    val
}

pub fn read_u32_le(data: &[u8], offset: &mut usize) -> u32 {
    let val = u32::from_le_bytes(data[*offset..*offset + 4].try_into().unwrap());
    *offset += 4;
    val
}

/// Reads a UTF-8 string prefixed with u32 length.
pub fn read_string(data: &[u8], offset: &mut usize) -> String {
    let len = read_u32_le(data, offset) as usize;
    let bytes = &data[*offset..*offset + len];
    *offset += len;
    from_utf8(bytes).unwrap().to_string()
}

pub fn read_pubkey(data: &[u8], offset: &mut usize) -> Pubkey {
    let key_bytes: [u8; 32] = data[*offset..*offset + 32]
        .try_into()
        .expect("slice with incorrect length");
    *offset += 32;
    Pubkey::new_from_array(key_bytes)
}