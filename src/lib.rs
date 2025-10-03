mod hamming;
mod hamming1511;
mod hamming74;

// Re-export
pub use hamming::Hamming;
pub use hamming74::Hamming74;
pub use hamming1511::Hamming1511;

#[derive(Debug, PartialEq)]
pub enum HammingError {
    InvalidLength,
    UncorrectableErrors,
}

pub trait HammingCode {
    /// Encode data into Hamming-encoded blocks
    fn encode(&self, data: &[u8]) -> Vec<u8>;

    /// Decode Hamming-encoded blocks back to data
    fn decode(&self, encoded: &[u8]) -> Result<Vec<u8>, HammingError>;

    /// Get the block size in bits for this code
    fn block_size(&self) -> usize;

    /// Get the data bits per block
    fn data_bits(&self) -> usize;
}
