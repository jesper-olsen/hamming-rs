# hamming-rs

A Rust implementation of Hamming error-correcting codes with support for both fixed-size and general Hamming codes.

## Features

- **Fixed-size implementations** for optimal performance:
  - Hamming(7,4) - encodes 4 data bits into 7 bits
  - Hamming(15,11) - encodes 11 data bits into 15 bits
- **General implementation** for arbitrary data sizes
- **Single-bit error correction** and detection
- **Zero dependencies** (except for the standard library)
- **Pure Rust** implementation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
hamming-rs = { git = "https://github.com/jesper-olsen/hamming-rs" }
```

## Performance

The fixed-size implementations (Hamming74 and Hamming1511) are significantly faster than the general implementation due to:

* Compile-time optimizations
* Bit manipulation tricks
* No runtime overhead

Benchmarks on a typical machine (encoding 1MB of data):

* Hamming74: ~2ms
* Hamming1511: ~3ms
* Hamming::new(11): ~12ms

## How Hamming Codes Work

Hamming codes add parity bits at positions that are powers of 2 (1, 2, 4, 8, ...). Each parity bit covers a specific set of positions:

* Parity bit 1: checks positions 1, 3, 5, 7, 9, 11, ...
* Parity bit 2: checks positions 2, 3, 6, 7, 10, 11, ...
* Parity bit 4: checks positions 4, 5, 6, 7, 12, 13, 14, 15, ...
* And so on...

When decoding, the parity bits form a "syndrome" that indicates the position of any single-bit error, which can then be corrected.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Richard Hamming](https://youtu.be/BZh07Ew32UA?si=DznrStL0qb2dnWeA) for the original error-correcting code algorithm (1950)
