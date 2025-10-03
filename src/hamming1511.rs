use crate::{HammingCode, HammingError};

/// Hamming(15,11) implementation
pub struct Hamming1511;

impl HammingCode for Hamming1511 {
    fn encode(&self, data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let mut encoded = Vec::new();
        let mut bit_accumulator = 0u32;
        let mut acc_bits = 0;

        for &byte in data {
            // Add byte to accumulator
            bit_accumulator |= (byte as u32) << acc_bits;
            acc_bits += 8;

            // Process while we have at least 11 bits
            while acc_bits >= 11 {
                // Take 11 bits
                let data_bits = (bit_accumulator & 0x7FF) as u16;
                bit_accumulator >>= 11;
                acc_bits -= 11;

                // Encode to 15 bits
                let encoded_block = Self::encode_block(data_bits);

                // Output the encoded block
                encoded.push(encoded_block as u8);
                encoded.push((encoded_block >> 8) as u8);
            }
        }

        // Handle remaining bits if any
        if acc_bits > 0 {
            let data_bits = (bit_accumulator & ((1 << acc_bits) - 1)) as u16;
            let encoded_block = Self::encode_block(data_bits);
            encoded.push(encoded_block as u8);
            encoded.push((encoded_block >> 8) as u8);
        }

        encoded
    }

    fn decode(&self, encoded: &[u8]) -> Result<Vec<u8>, HammingError> {
        if encoded.is_empty() {
            return Ok(Vec::new());
        }

        if encoded.len() % 2 != 0 {
            return Err(HammingError::InvalidLength);
        }

        let mut decoded = Vec::new();
        let mut bit_accumulator = 0u32;
        let mut acc_bits = 0;

        // Process each 15-bit block (stored in 2 bytes)
        for chunk in encoded.chunks(2) {
            let block = chunk[0] as u16 | ((chunk[1] as u16) << 8);

            // Decode the block
            let data_bits = Self::decode_block(block)?;

            // Add to accumulator
            bit_accumulator |= (data_bits as u32) << acc_bits;
            acc_bits += 11;

            // Output complete bytes
            while acc_bits >= 8 {
                decoded.push(bit_accumulator as u8);
                bit_accumulator >>= 8;
                acc_bits -= 8;
            }
        }

        Ok(decoded)
    }

    fn block_size(&self) -> usize {
        15
    }
    fn data_bits(&self) -> usize {
        11
    }
}

impl Hamming1511 {
    fn encode_block(data: u16) -> u16 {
        let d = data & 0x7FF; // Ensure only 11 bits

        // Map data bits to their positions in the 15-bit block
        // Hamming positions (1-indexed): 1,2,4,8 are parity
        // Data positions (1-indexed): 3,5,6,7,9,10,11,12,13,14,15
        let mut block = 0u16;

        // Place data bits
        block |= (d & 0x001) << 2; // d0 -> position 3 (bit 2)
        block |= (d & 0x002) << 3; // d1 -> position 5 (bit 4)
        block |= (d & 0x004) << 3; // d2 -> position 6 (bit 5)
        block |= (d & 0x008) << 3; // d3 -> position 7 (bit 6)
        block |= (d & 0x010) << 4; // d4 -> position 9 (bit 8)
        block |= (d & 0x020) << 4; // d5 -> position 10 (bit 9)
        block |= (d & 0x040) << 4; // d6 -> position 11 (bit 10)
        block |= (d & 0x080) << 4; // d7 -> position 12 (bit 11)
        block |= (d & 0x100) << 4; // d8 -> position 13 (bit 12)
        block |= (d & 0x200) << 4; // d9 -> position 14 (bit 13)
        block |= (d & 0x400) << 4; // d10 -> position 15 (bit 14)

        // Calculate and set parity bits
        let p1 = Self::calc_parity(block, 0x5555); // positions with bit 0 set
        let p2 = Self::calc_parity(block, 0x6666); // positions with bit 1 set
        let p4 = Self::calc_parity(block, 0x7878); // positions with bit 2 set
        let p8 = Self::calc_parity(block, 0x7F80); // positions with bit 3 set

        block |= p1; // position 1 (bit 0)
        block |= p2 << 1; // position 2 (bit 1)
        block |= p4 << 3; // position 4 (bit 3)
        block |= p8 << 7; // position 8 (bit 7)

        block
    }

    fn decode_block(block: u16) -> Result<u16, HammingError> {
        // Calculate syndrome
        let s1 = Self::calc_parity(block, 0x5555);
        let s2 = Self::calc_parity(block, 0x6666);
        let s4 = Self::calc_parity(block, 0x7878);
        let s8 = Self::calc_parity(block, 0x7F80);

        let syndrome = s1 | (s2 << 1) | (s4 << 2) | (s8 << 3);

        // Correct error if needed
        let mut corrected = block;
        if syndrome != 0 {
            if syndrome <= 15 {
                corrected ^= 1 << (syndrome - 1);
            } else {
                return Err(HammingError::UncorrectableErrors);
            }
        }

        // Extract data bits from corrected block
        let mut data = 0u16;
        data |= (corrected >> 2) & 0x001; // position 3 -> d0
        data |= (corrected >> 3) & 0x002; // position 5 -> d1
        data |= (corrected >> 3) & 0x004; // position 6 -> d2
        data |= (corrected >> 3) & 0x008; // position 7 -> d3
        data |= (corrected >> 4) & 0x010; // position 9 -> d4
        data |= (corrected >> 4) & 0x020; // position 10 -> d5
        data |= (corrected >> 4) & 0x040; // position 11 -> d6
        data |= (corrected >> 4) & 0x080; // position 12 -> d7
        data |= (corrected >> 4) & 0x100; // position 13 -> d8
        data |= (corrected >> 4) & 0x200; // position 14 -> d9
        data |= (corrected >> 4) & 0x400; // position 15 -> d10

        Ok(data)
    }

    #[inline]
    fn calc_parity(block: u16, mask: u16) -> u16 {
        (block & mask).count_ones() as u16 & 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming1511_basic() {
        let h = Hamming1511;
        let data = vec![0x47, 0xA3];

        let encoded = h.encode(&data);
        let decoded = h.decode(&encoded).unwrap();

        assert_eq!(decoded, data);
    }

    #[test]
    fn test_hamming1511_single_bit_error() {
        let h = Hamming1511;
        let data = vec![0x55, 0xAA];

        let mut encoded = h.encode(&data);
        // Introduce an error in the first block
        encoded[0] ^= 0x20;

        let decoded = h.decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_hamming1511_block_encoding() {
        // Test specific bit pattern
        let data = 0x555; // 01010101101 (11 bits)
        let encoded = Hamming1511::encode_block(data);
        let decoded = Hamming1511::decode_block(encoded).unwrap();

        assert_eq!(decoded, data);
    }

    #[test]
    fn test_hamming1511_all_ones() {
        let data = 0x7FF; // All 11 bits set
        let encoded = Hamming1511::encode_block(data);
        let decoded = Hamming1511::decode_block(encoded).unwrap();

        assert_eq!(decoded, data);
    }
}
