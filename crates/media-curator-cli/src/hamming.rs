//! Pure Hamming-distance core — parity target for `src/comparatorUtils.ts`.

/// Population count for a single byte (parity with TS `popcount8`).
#[must_use]
pub fn popcount8(mut n: u8) -> u32 {
    n = n.wrapping_sub((n >> 1) & 0x55);
    n = (n & 0x33).wrapping_add((n >> 2) & 0x33);
    u32::from((n.wrapping_add(n >> 4)) & 0x0f)
}

/// Population count for a 64-bit word (parity with TS `popcount64`).
#[must_use]
pub fn popcount64(mut n: u64) -> u32 {
    n = n.wrapping_sub((n >> 1) & 0x5555_5555_5555_5555);
    n = (n & 0x3333_3333_3333_3333).wrapping_add((n >> 2) & 0x3333_3333_3333_3333);
    n = (n.wrapping_add(n >> 4)) & 0x0f0f_0f0f_0f0f_0f0f;
    n = n.wrapping_add(n >> 8);
    n = n.wrapping_add(n >> 16);
    n = n.wrapping_add(n >> 32);
    (n & 0x7f) as u32
}

/// Hamming distance between two byte slices (XOR + popcount).
/// Uses the shorter length when lengths differ (TS minLen behavior).
#[must_use]
pub fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    let min_len = a.len().min(b.len());
    let mut distance = 0u32;
    let common_chunks = min_len / 8;
    for i in 0..common_chunks {
        let off = i * 8;
        let wa = u64::from_le_bytes(a[off..off + 8].try_into().unwrap_or([0; 8]));
        let wb = u64::from_le_bytes(b[off..off + 8].try_into().unwrap_or([0; 8]));
        distance += popcount64(wa ^ wb);
    }
    let start = common_chunks * 8;
    for i in start..min_len {
        distance += popcount8(a[i] ^ b[i]);
    }
    // Remaining bytes in the longer side count as differing bits (full popcount).
    let longer = if a.len() > b.len() { a } else { b };
    for &byte in longer.iter().skip(min_len) {
        distance += popcount8(byte);
    }
    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identical_hashes_distance_zero() {
        let a = [0u8; 16];
        assert_eq!(hamming_distance(&a, &a), 0);
    }

    #[test]
    fn single_bit_flip() {
        let a = [0u8; 8];
        let mut b = [0u8; 8];
        b[0] = 0b0000_0001;
        assert_eq!(hamming_distance(&a, &b), 1);
    }

    #[test]
    fn popcount8_known() {
        assert_eq!(popcount8(0), 0);
        assert_eq!(popcount8(0xff), 8);
        assert_eq!(popcount8(0b1010_1010), 4);
    }
}
