use crate::{HammingCode, HammingError};

/// General Hamming code implementation
pub struct Hamming {
    data_bits: usize,
    parity_bits: usize,
}

impl Hamming {
    pub fn new(data_bits: usize) -> Self {
        // Calculate required parity bits: 2^r >= m + r + 1
        let mut parity_bits = 1;
        while (1 << parity_bits) < data_bits + parity_bits + 1 {
            parity_bits += 1;
        }

        Self {
            data_bits,
            parity_bits,
        }
    }
}

impl HammingCode for Hamming {
    fn encode(&self, data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let block_bits = self.data_bits + self.parity_bits;
        let total_data_bits = data.len() * 8;

        // Calculate number of blocks needed
        let num_blocks = (total_data_bits + self.data_bits - 1) / self.data_bits;

        // IMPORTANT: Calculate the exact output size
        let total_output_bits = num_blocks * block_bits;
        let output_bytes = (total_output_bits + 7) / 8;

        let mut encoded = vec![0u8; output_bytes];

        // Process data in chunks
        for block_idx in 0..num_blocks {
            // Start position for this block in the output
            let output_bit_offset = block_idx * block_bits;

            // Initialize block with all zeros
            let mut block = vec![false; block_bits];

            // Fill in data bits
            let data_start_bit = block_idx * self.data_bits;
            let mut data_bit_count = 0;

            for pos in 1..=block_bits {
                if !pos.is_power_of_two() && data_bit_count < self.data_bits {
                    let global_data_bit = data_start_bit + data_bit_count;
                    if global_data_bit < total_data_bits {
                        let byte_idx = global_data_bit / 8;
                        let bit_idx = global_data_bit % 8;
                        block[pos - 1] = (data[byte_idx] >> bit_idx) & 1 == 1;
                    }
                    data_bit_count += 1;
                }
            }

            // Calculate parity bits
            for p in 0..self.parity_bits {
                let parity_pos = 1 << p;
                let mut parity = false;

                for i in 1..=block_bits {
                    if (i & parity_pos) != 0 && block[i - 1] {
                        parity = !parity;
                    }
                }

                block[parity_pos - 1] = parity;
            }

            // Write block to output
            for (i, &bit) in block.iter().enumerate() {
                if bit {
                    let global_bit_pos = output_bit_offset + i;
                    let byte_idx = global_bit_pos / 8;
                    let bit_idx = global_bit_pos % 8;
                    encoded[byte_idx] |= 1 << bit_idx;
                }
            }
        }

        encoded
    }

    fn decode(&self, encoded: &[u8]) -> Result<Vec<u8>, HammingError> {
        if encoded.is_empty() {
            return Ok(Vec::new());
        }

        let block_bits = self.data_bits + self.parity_bits;
        let total_bits = encoded.len() * 8;

        let num_blocks = total_bits / block_bits;
        if num_blocks == 0 {
            return Err(HammingError::InvalidLength);
        }

        let num_blocks = total_bits / block_bits;
        let total_data_bits = num_blocks * self.data_bits;
        let output_bytes = (total_data_bits + 7) / 8;

        let mut decoded = vec![0u8; output_bytes];
        let mut decoded_bit_pos = 0;

        for block_idx in 0..num_blocks {
            // Read one block
            let block_start_bit = block_idx * block_bits;
            let mut block = vec![false; block_bits];

            for i in 0..block_bits {
                let global_bit = block_start_bit + i;
                let byte_idx = global_bit / 8;
                let bit_idx = global_bit % 8;
                block[i] = (encoded[byte_idx] >> bit_idx) & 1 == 1;
            }

            // Calculate syndrome
            let mut syndrome = 0;
            for p in 0..self.parity_bits {
                let parity_pos = 1 << p;
                let mut calculated_parity = false;

                for i in 1..=block_bits {
                    if (i & parity_pos) != 0 && block[i - 1] {
                        calculated_parity = !calculated_parity;
                    }
                }

                if calculated_parity {
                    syndrome |= parity_pos;
                }
            }

            // Fix single-bit error if needed
            if syndrome != 0 && syndrome <= block_bits {
                block[syndrome - 1] = !block[syndrome - 1];
            } else if syndrome > block_bits {
                return Err(HammingError::UncorrectableErrors);
            }

            // Extract data bits
            for pos in 1..=block_bits {
                if !pos.is_power_of_two() && decoded_bit_pos < total_data_bits {
                    if block[pos - 1] {
                        let byte_idx = decoded_bit_pos / 8;
                        let bit_idx = decoded_bit_pos % 8;
                        decoded[byte_idx] |= 1 << bit_idx;
                    }
                    decoded_bit_pos += 1;
                }
            }
        }

        Ok(decoded)
    }

    fn block_size(&self) -> usize {
        self.data_bits + self.parity_bits
    }

    fn data_bits(&self) -> usize {
        self.data_bits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general_hamming() {
        let h = Hamming::new(11); // Hamming(15,11)
        let data = vec![0x47, 0xA3]; // 16 bits

        let encoded = h.encode(&data);
        let decoded = h.decode(&encoded).unwrap();

        // With padding, we might get extra zeros
        assert!(decoded.starts_with(&data));
    }

    #[test]
    fn test_general_hamming_exact_fit() {
        let h = Hamming::new(4); // Like Hamming(7,4)
        let data = vec![0x55]; // 8 bits = exactly 2 blocks of 4 bits each

        let encoded = h.encode(&data);
        assert_eq!(encoded.len(), 2); // 2 blocks * 7 bits = 14 bits = 2 bytes

        let decoded = h.decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }
}
