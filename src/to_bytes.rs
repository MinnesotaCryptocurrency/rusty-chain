pub fn u32_bytes (u: &u32) -> [u8; 4] {
    [
        (u >> 8 * 0) as u8,
        (u >> 8 * 1) as u8,
        (u >> 8 * 2) as u8,
        (u >> 8 * 3) as u8,
    ]
}

pub fn u64_bytes (u: &u64) -> [u8; 8] {
    [
        (u >> 8 * 0) as u8,
        (u >> 8 * 1) as u8,
        (u >> 8 * 2) as u8,
        (u >> 8 * 3) as u8,

        (u >> 8 * 4) as u8,
        (u >> 8 * 5) as u8,
        (u >> 8 * 6) as u8,
        (u >> 8 * 7) as u8,
    ]
}
