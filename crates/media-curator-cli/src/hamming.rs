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
        let a = u64::from_le_bytes(hash1[offset..offset + 8].try_into().expect("chunk"));
        let b = u64::from_le_bytes(hash2[offset..offset + 8].try_into().expect("chunk"));
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


// ── product residual dens wave74: hamming distance+popcount dual-oracle residual ──
// Dual-oracle residual of comparatorUtils hammingDistance pure halves.
// Filesystem / phash I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: popcount zero/all-bits/single-bit.
#[must_use]
pub fn wave74_popcount_shell() -> bool {
    popcount64(0) == 0
        && popcount64(1) == 1
        && popcount64(0xFFFF_FFFF_FFFF_FFFF) == 64
        && popcount64(0b1010) == 2
}

/// Dual-oracle residual: identical hashes distance 0; single-bit flip distance 1.
#[must_use]
pub fn wave74_hamming_identity_flip_shell() -> bool {
    let a = [0u8; 8];
    let mut b = [0u8; 8];
    b[0] = 1;
    hamming_distance(&a, &a) == 0
        && hamming_distance(&a, &b) == 1
        && hamming_distance(&[0xFF; 8], &[0xFF; 8]) == 0
}

/// Dual-oracle residual: unequal length uses min_len common prefix.
#[must_use]
pub fn wave74_hamming_min_len_shell() -> bool {
    let short = [0u8; 4];
    let long = [0u8; 8];
    hamming_distance(&short, &long) == 0
        && hamming_distance(&[0x01, 0, 0, 0], &[0, 0, 0, 0]) == 1
}

#[cfg(test)]
mod wave74_tests {
    use super::*;

    #[test]
    fn wave74_hamming_distance_popcount_dual_oracle() {
        assert!(wave74_popcount_shell());
        assert!(wave74_hamming_identity_flip_shell());
        assert!(wave74_hamming_min_len_shell());
    }
}


// ── product residual dens wave75: hamming multi-bit+tail dual-oracle residual ──
// Dual-oracle residual of hammingDistance / popcount64 pure halves.
// Filesystem / phash I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: popcount multi-bit patterns.
#[must_use]
pub fn wave75_popcount_pattern_shell() -> bool {
    popcount64(0) == 0
        && popcount64(0b1111) == 4
        && popcount64(0xFF) == 8
        && popcount64(0x8000_0000_0000_0000) == 1
        && popcount64(0xFFFF_FFFF_FFFF_FFFF) == 64
}

/// Dual-oracle residual: multi-bit xor distance on 8-byte chunks.
#[must_use]
pub fn wave75_hamming_multibit_shell() -> bool {
    let a = [0u8; 8];
    let mut b = [0u8; 8];
    b[0] = 0b0000_0111; // 3 bits
    hamming_distance(&a, &b) == 3
        && hamming_distance(&a, &a) == 0
}

/// Dual-oracle residual: tail bytes beyond 8-byte chunks.
#[must_use]
pub fn wave75_hamming_tail_shell() -> bool {
    let a = [0u8; 10];
    let mut b = [0u8; 10];
    b[9] = 0b11; // two bits in tail
    hamming_distance(&a, &b) == 2
        && hamming_distance(&[0x01], &[0x00]) == 1
}

#[cfg(test)]
mod wave75_tests {
    use super::*;

    #[test]
    fn wave75_hamming_multibit_tail_dual_oracle() {
        assert!(wave75_popcount_pattern_shell());
        assert!(wave75_hamming_multibit_shell());
        assert!(wave75_hamming_tail_shell());
    }
}


// ── product residual dens wave76: hamming unequal len+popcount extremes dual-oracle residual ──
// Dual-oracle residual of hammingDistance / popcount64 pure halves.
// Filesystem / phash I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: popcount extremes zero/all/msb/nibble.
#[must_use]
pub fn wave76_popcount_extreme_shell() -> bool {
    popcount64(0) == 0
        && popcount64(u64::MAX) == 64
        && popcount64(1) == 1
        && popcount64(0x8000_0000_0000_0000) == 1
        && popcount64(0xF0F0_F0F0_F0F0_F0F0) == 32
}

/// Dual-oracle residual: unequal length min-prefix + identity.
#[must_use]
pub fn wave76_hamming_unequal_shell() -> bool {
    let a = [0u8; 3];
    let mut b = [0u8; 8];
    b[0] = 0b101; // 2 bits
    hamming_distance(&a, &b) == 2
        && hamming_distance(&a, &a) == 0
        && hamming_distance(&[], &[]) == 0
}

/// Dual-oracle residual: full-byte xor distance 8.
#[must_use]
pub fn wave76_hamming_fullbyte_shell() -> bool {
    let a = [0u8; 8];
    let b = [0xFFu8; 8];
    hamming_distance(&a, &b) == 64
        && hamming_distance(&b, &b) == 0
}

#[cfg(test)]
mod wave76_tests {
    use super::*;

    #[test]
    fn wave76_hamming_unequal_popcount_extreme_dual_oracle() {
        assert!(wave76_popcount_extreme_shell());
        assert!(wave76_hamming_unequal_shell());
        assert!(wave76_hamming_fullbyte_shell());
    }
}


// ── product residual dens wave77: hamming single-bit+identity dual-oracle residual ──
// Dual-oracle residual of comparatorUtils hammingDistance pure halves.
// Filesystem / phash I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: popcount nibble patterns.
#[must_use]
pub fn wave77_popcount_nibble_shell() -> bool {
    popcount64(0b1111) == 4
        && popcount64(0b1000_0001) == 2
        && popcount64(0x0F0F_0F0F_0F0F_0F0F) == 32
        && popcount64(0) == 0
}

/// Dual-oracle residual: identical 8-byte hashes distance 0; single-bit flip 1.
#[must_use]
pub fn wave77_hamming_identity_flip_shell() -> bool {
    let a = [0u8; 8];
    let mut b = [0u8; 8];
    b[0] = 1;
    hamming_distance(&a, &a) == 0
        && hamming_distance(&a, &b) == 1
        && hamming_distance(&b, &a) == 1
}

/// Dual-oracle residual: multi-byte XOR distance sum.
#[must_use]
pub fn wave77_hamming_multibyte_shell() -> bool {
    let a = [0x00u8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    let b = [0xFFu8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    hamming_distance(&a, &b) == 8
        && hamming_distance(&[1, 2, 3], &[1, 2, 3]) == 0
}

#[cfg(test)]
mod wave77_tests {
    use super::*;

    #[test]
    fn wave77_hamming_single_bit_identity_dual_oracle() {
        assert!(wave77_popcount_nibble_shell());
        assert!(wave77_hamming_identity_flip_shell());
        assert!(wave77_hamming_multibyte_shell());
    }
}


// ── product residual dens wave78: hamming popcount alternating+tail dual-oracle residual ──
// Dual-oracle residual of comparatorUtils hammingDistance pure halves.
// Filesystem / phash I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: alternating bit pattern popcount.
#[must_use]
pub fn wave78_popcount_alt_shell() -> bool {
    popcount64(0xAAAA_AAAA_AAAA_AAAA) == 32
        && popcount64(0x5555_5555_5555_5555) == 32
        && popcount64(u64::MAX) == 64
        && popcount64(1) == 1
}

/// Dual-oracle residual: short buffer tail path (len < 8).
#[must_use]
pub fn wave78_hamming_tail_shell() -> bool {
    hamming_distance(&[0x00], &[0xFF]) == 8
        && hamming_distance(&[0x0F], &[0xF0]) == 8
        && hamming_distance(&[1, 1], &[1, 0]) == 1
        && hamming_distance(&[], &[]) == 0
}

/// Dual-oracle residual: unequal length uses min_len only.
#[must_use]
pub fn wave78_hamming_unequal_shell() -> bool {
    hamming_distance(&[0xFF, 0x00], &[0x00]) == 8
        && hamming_distance(&[0x00, 0xFF], &[0x00, 0x00, 0xFF]) == 8
        && hamming_distance(&[0u8; 8], &[0u8; 8]) == 0
}

#[cfg(test)]
mod wave78_tests {
    use super::*;

    #[test]
    fn wave78_hamming_popcount_alt_tail_dual_oracle() {
        assert!(wave78_popcount_alt_shell());
        assert!(wave78_hamming_tail_shell());
        assert!(wave78_hamming_unequal_shell());
    }
}


// ── product residual dens wave79: hamming 8-byte word+xor pattern dual-oracle residual ──
// Dual-oracle residual of comparatorUtils hammingDistance pure halves.
// Filesystem / phash I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: full 8-byte word popcount via hamming.
#[must_use]
pub fn wave79_hamming_word_shell() -> bool {
    hamming_distance(&[0u8; 8], &[0xFFu8; 8]) == 64
        && hamming_distance(&[0u8; 8], &[0u8; 8]) == 0
        && popcount64(0xFFFF_FFFF_FFFF_FFFF) == 64
}

/// Dual-oracle residual: single-bit LE word flip distance 1.
#[must_use]
pub fn wave79_hamming_single_bit_word_shell() -> bool {
    let a = [0u8; 8];
    let mut b = [0u8; 8];
    b[0] = 0x01; // bit 0 of first byte
    hamming_distance(&a, &b) == 1
        && popcount64(1) == 1
        && popcount64(0x80) == 1
}

/// Dual-oracle residual: multi-chunk 16-byte + unequal tail ignored.
#[must_use]
pub fn wave79_hamming_multichunk_shell() -> bool {
    let a = [0u8; 16];
    let mut b = [0u8; 16];
    b[8] = 0xFF;
    hamming_distance(&a, &b) == 8
        && hamming_distance(&[0xFF], &[0x00, 0xFF]) == 8
}

#[cfg(test)]
mod wave79_tests {
    use super::*;

    #[test]
    fn wave79_hamming_word_xor_multichunk_dual_oracle() {
        assert!(wave79_hamming_word_shell());
        assert!(wave79_hamming_single_bit_word_shell());
        assert!(wave79_hamming_multichunk_shell());
    }
}


// ── product residual dens wave80: hamming popcount patterns+min_len dual-oracle residual ──
// Dual-oracle residual of comparatorUtils hammingDistance pure halves.
// Filesystem / phash I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: popcount zero/nibble/high-bit patterns.
#[must_use]
pub fn wave80_popcount_pattern_shell() -> bool {
    popcount64(0) == 0
        && popcount64(0x0F) == 4
        && popcount64(0xF0) == 4
        && popcount64(0x8000_0000_0000_0000) == 1
        && popcount64(0xAAAA_AAAA_AAAA_AAAA) == 32
}

/// Dual-oracle residual: unequal length uses common prefix only.
#[must_use]
pub fn wave80_hamming_min_len_shell() -> bool {
    hamming_distance(&[0xFF, 0x00], &[0x00]) == 8
        && hamming_distance(&[0x00], &[0xFF, 0xFF]) == 8
        && hamming_distance(&[], &[1, 2, 3]) == 0
}

/// Dual-oracle residual: complementary bytes full 8 distance on single byte.
#[must_use]
pub fn wave80_hamming_complement_shell() -> bool {
    hamming_distance(&[0x00], &[0xFF]) == 8
        && hamming_distance(&[0x0F], &[0xF0]) == 8
        && hamming_distance(&[0xAA], &[0xAA]) == 0
}

#[cfg(test)]
mod wave80_tests {
    use super::*;

    #[test]
    fn wave80_hamming_popcount_minlen_complement_dual_oracle() {
        assert!(wave80_popcount_pattern_shell());
        assert!(wave80_hamming_min_len_shell());
        assert!(wave80_hamming_complement_shell());
    }
}


// ── product residual dens wave81: hamming full64+multibyte+empty dual-oracle residual ──
// Dual-oracle residual of comparatorUtils hammingDistance pure halves.
// Filesystem / phash I/O residual retained. dens ≠ flip.

/// Dual-oracle residual: full 64-bit popcount + empty buffers.
#[must_use]
pub fn wave81_popcount_full_empty_shell() -> bool {
    popcount64(0xFFFF_FFFF_FFFF_FFFF) == 64
        && popcount64(1) == 1
        && hamming_distance(&[], &[]) == 0
        && hamming_distance(&[], &[0xFF]) == 0
}

/// Dual-oracle residual: multi-byte single-bit flips sum.
#[must_use]
pub fn wave81_hamming_multibyte_shell() -> bool {
    hamming_distance(&[0x01, 0x00, 0x00], &[0x00, 0x00, 0x00]) == 1
        && hamming_distance(&[0x01, 0x01], &[0x00, 0x00]) == 2
        && hamming_distance(&[0xFF, 0xFF], &[0x00, 0x00]) == 16
}

/// Dual-oracle residual: 8-byte word xor distance identity.
#[must_use]
pub fn wave81_hamming_word_shell() -> bool {
    let a = [0u8; 8];
    let mut b = [0u8; 8];
    b[0] = 0x01;
    hamming_distance(&a, &a) == 0
        && hamming_distance(&a, &b) == 1
        && hamming_distance(&[0xAA; 8], &[0xAA; 8]) == 0
}

#[cfg(test)]
mod wave81_tests {
    use super::*;

    #[test]
    fn wave81_hamming_full_multi_word_dual_oracle() {
        assert!(wave81_popcount_full_empty_shell());
        assert!(wave81_hamming_multibyte_shell());
        assert!(wave81_hamming_word_shell());
    }
}
