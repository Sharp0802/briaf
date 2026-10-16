pub const fn splitmix(x: usize) -> usize {
    #![allow(clippy::cast_possible_truncation)]
    #[cfg(target_pointer_width = "64")]
    return splitmix64(x as u64) as usize;
    #[cfg(target_pointer_width = "32")]
    return splitmix32(x as u32) as usize;
}

#[cfg(target_pointer_width = "64")]
const fn splitmix64(x: u64) -> u64 {
    let z = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
    let z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    let z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

#[cfg(target_pointer_width = "32")]
const fn splitmix32(x: u32) -> u32 {
    let z = x.wrapping_add(0x9e37_79b9);
    let z = (z ^ (z >> 15)).wrapping_mul(0x85eb_ca6b);
    let z = (z ^ (z >> 13)).wrapping_mul(0xc2b2_ae35);
    z ^ (z >> 16)
}
