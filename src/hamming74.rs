use crate::{HammingCode, HammingError};

/// Hamming(7,4) implementation
pub struct Hamming74;

impl HammingCode for Hamming74 {
    fn encode(&self, data: &[u8]) -> Vec<u8> {
        let mut encoded = Vec::new();

        for byte in data {
            // Process lower nibble
            encoded.push(Self::encode_nibble(*byte & 0x0F));
            // Process upper nibble
            encoded.push(Self::encode_nibble(*byte >> 4));
        }

        encoded
    }

    fn decode(&self, encoded: &[u8]) -> Result<Vec<u8>, HammingError> {
        if encoded.len() % 2 != 0 {
            return Err(HammingError::InvalidLength);
        }

        let mut decoded = Vec::new();

        for pair in encoded.chunks(2) {
            let lower = Self::decode_block(pair[0])?;
            let upper = Self::decode_block(pair[1])?;
            decoded.push(lower | (upper << 4));
        }

        Ok(decoded)
    }

    fn block_size(&self) -> usize {
        7
    }

    fn data_bits(&self) -> usize {
        4
    }
}

impl Hamming74 {
    fn encode_nibble(nibble: u8) -> u8 {
        let d1 = (nibble >> 0) & 1;
        let d2 = (nibble >> 1) & 1;
        let d3 = (nibble >> 2) & 1;
        let d4 = (nibble >> 3) & 1;

        // Calculate parity bits
        let p1 = d1 ^ d2 ^ d4;
        let p2 = d1 ^ d3 ^ d4;
        let p3 = d2 ^ d3 ^ d4;

        // Layout: p1 p2 d1 p3 d2 d3 d4
        (p1 << 0) | (p2 << 1) | (d1 << 2) | (p3 << 3) | (d2 << 4) | (d3 << 5) | (d4 << 6)
    }

    fn decode_block(block: u8) -> Result<u8, HammingError> {
        let block = block & 0x7F; // Only use lower 7 bits

        // Calculate syndrome
        let s1 = ((block >> 0) & 1) ^ ((block >> 2) & 1) ^ ((block >> 4) & 1) ^ ((block >> 6) & 1);
        let s2 = ((block >> 1) & 1) ^ ((block >> 2) & 1) ^ ((block >> 5) & 1) ^ ((block >> 6) & 1);
        let s3 = ((block >> 3) & 1) ^ ((block >> 4) & 1) ^ ((block >> 5) & 1) ^ ((block >> 6) & 1);

        let syndrome = s1 | (s2 << 1) | (s3 << 2);

        // Correct single bit error if needed
        let mut corrected = block;
        if syndrome != 0 {
            let error_pos = syndrome - 1;
            if error_pos < 7 {
                corrected ^= 1 << error_pos;
            } else {
                return Err(HammingError::UncorrectableErrors);
            }
        }

        // Extract data bits from positions 2, 4, 5, 6
        let d1 = (corrected >> 2) & 1;
        let d2 = (corrected >> 4) & 1;
        let d3 = (corrected >> 5) & 1;
        let d4 = (corrected >> 6) & 1;

        Ok(d1 | (d2 << 1) | (d3 << 2) | (d4 << 3))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hamming74::Hamming74;

    #[test]
    fn test_hamming74_encode_decode() {
        let h74 = Hamming74;
        let data = vec![0x47, 0xA3]; // Example data

        let encoded = h74.encode(&data);
        let decoded = h74.decode(&encoded).unwrap();

        assert_eq!(data, decoded);
    }

    #[test]
    fn test_hamming74_single_bit_error() {
        let h74 = Hamming74;
        let data = vec![0x47];

        let mut encoded = h74.encode(&data);
        // Flip a bit
        encoded[0] ^= 0x08;

        let decoded = h74.decode(&encoded).unwrap();
        assert_eq!(data, decoded);
    }
}
