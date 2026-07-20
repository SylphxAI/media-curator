//! Hamming distance — parity with `src/comparatorUtils.ts#hammingDistance` JS fallback.

/// Popcount for 64-bit integers (TS `popcount64` parity).
#[must_use]
pub fn popcount64(value: u64) -> u32 {
    let mut n = value;
    n = n.wrapping_sub((n >> 1) & 0x5555_5555_5555_5555);
    n = (n & 0x3333_3333_3333_3333) + ((n >> 2) & 0x3333_3333_3333_3333);
    n = (n + (n >> 4)) & 0x0f0f_0f0f_0f0f_0f0f;
    n = n + (n >> 8);
    n = n + (n >> 16);
    n = n + (n >> 32);
    (n & 0x7f) as u32
}

/// Hamming distance between two byte buffers (JS fallback path).
#[must_use]
pub fn hamming_distance(hash1: &[u8], hash2: &[u8]) -> u64 {
    let min_len = hash1.len().min(hash2.len());
    let common_chunks = min_len / 8;
    let mut distance = 0u64;

    for chunk in 0..common_chunks {
        let offset = chunk * 8;
        let mut a_bytes = [0u8; 8];
        let mut b_bytes = [0u8; 8];
        a_bytes.copy_from_slice(&hash1[offset..offset + 8]);
        b_bytes.copy_from_slice(&hash2[offset..offset + 8]);
        let a = u64::from_le_bytes(a_bytes);
        let b = u64::from_le_bytes(b_bytes);
        distance += u64::from(popcount64(a ^ b));
    }

    let start = common_chunks * 8;
    for index in start..min_len {
        distance += (hash1[index] ^ hash2[index]).count_ones() as u64;
    }

    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_hashes_have_zero_distance() {
        let hash = [0xFF_u8; 8];
        assert_eq!(hamming_distance(&hash, &hash), 0);
    }
}
