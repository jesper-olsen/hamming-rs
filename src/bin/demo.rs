use hamming_rs::{Hamming, Hamming74, Hamming1511, HammingCode};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!("Hamming Code Demo");
    println!(
        "Commands: '74' for Hamming(7,4), '1511' for Hamming(15,11), 'general' for general Hamming, 'quit' to exit\n"
    );

    let stdin = io::stdin();
    let mut current_hamming: Box<dyn HammingCode> = Box::new(Hamming74);

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let input = input.trim();

        match input {
            "quit" | "exit" => break,
            "74" => {
                current_hamming = Box::new(Hamming74);
                println!("Switched to Hamming(7,4)");
            }
            "1511" => {
                current_hamming = Box::new(Hamming1511);
                println!("Switched to Hamming(15,11)");
            }
            "general" => {
                print!("Enter data bits (e.g., 11 for Hamming(15,11)): ");
                io::stdout().flush()?;

                let mut bits_input = String::new();
                stdin.read_line(&mut bits_input)?;

                if let Ok(data_bits) = bits_input.trim().parse::<usize>() {
                    current_hamming = Box::new(Hamming::new(data_bits));
                    println!("Switched to Hamming with {} data bits", data_bits);
                } else {
                    println!("Invalid number");
                }
            }
            "" => continue,
            text => {
                // Encode the text
                let data = text.as_bytes(); // Here's the &[u8] conversion!
                println!("\nOriginal: \"{}\" ({} bytes)", text, data.len());
                println!("Bytes: {:02X?}", data);

                let encoded = current_hamming.encode(data);
                println!("\nEncoded: {} bytes", encoded.len());
                println!("Bytes: {:02X?}", encoded);

                // Simulate error
                print!("\nIntroduce error? (y/n): ");
                io::stdout().flush()?;

                let mut error_input = String::new();
                stdin.read_line(&mut error_input)?;

                let mut encoded_with_error = encoded.clone();
                if error_input.trim().eq_ignore_ascii_case("y") && !encoded_with_error.is_empty() {
                    print!("Byte position (0-{}): ", encoded_with_error.len() - 1);
                    io::stdout().flush()?;

                    let mut pos_input = String::new();
                    stdin.read_line(&mut pos_input)?;

                    if let Ok(pos) = pos_input.trim().parse::<usize>()
                        && pos < encoded_with_error.len()
                    {
                        print!("Bit position (0-7): ");
                        io::stdout().flush()?;

                        let mut bit_input = String::new();
                        stdin.read_line(&mut bit_input)?;

                        if let Ok(bit) = bit_input.trim().parse::<u8>()
                            && bit < 8
                        {
                            encoded_with_error[pos] ^= 1 << bit;
                            println!("Flipped bit {} in byte {}", bit, pos);
                            println!("Corrupted: {:02X?}", encoded_with_error);
                        }
                    }
                }

                // Decode
                match current_hamming.decode(&encoded_with_error) {
                    Ok(decoded) => {
                        println!("\nDecoded bytes: {:02X?}", decoded);

                        // Try to convert back to string
                        match String::from_utf8(decoded.clone()) {
                            Ok(text) => println!("Decoded text: \"{}\"", text),
                            Err(_) => println!("Decoded data (not valid UTF-8): {:?}", decoded),
                        }
                    }
                    Err(e) => {
                        println!("Decode error: {:?}", e);
                    }
                }

                println!();
            }
        }
    }

    println!("Goodbye!");
    Ok(())
}
