use bs58;
use std::io::{self, BufRead};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn base58_to_wallet() {
        println!("Input your private key as a base58 string:");
        let stdin = io::stdin();
        if let Some(Ok(base58)) = stdin.lock().lines().next() {
            if !base58.trim().is_empty() {
                if let Ok(wallet) = bs58::decode(base58.trim()).into_vec() {
                    println!("Your wallet file format is:");
                    println!("{:?}", wallet);
                } else {
                    println!("Invalid Base58 string");
                }
            }
        }
    }

    #[test]
    #[ignore]
    fn wallet_to_base58() {
        println!("Input your private key as a JSON byte array (e.g. [12,34,...]):");
        let stdin = io::stdin();
        if let Some(Ok(input)) = stdin.lock().lines().next() {
            if !input.trim().is_empty() {
                let bytes: Result<Vec<u8>, _> = input
                    .trim()
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .split(',')
                    .map(|s| s.trim().parse::<u8>())
                    .collect();
                
                if let Ok(wallet) = bytes {
                    let base58 = bs58::encode(wallet).into_string();
                    println!("Your Base58-encoded private key is:");
                    println!("{}", base58);
                }
            }
        }
    }

    #[test]
    fn conversion_demo() {
        let wallet_bytes = vec![154, 124, 192, 202, 110, 92, 226, 203, 211, 117, 131, 4, 103, 150, 175, 11, 76, 63, 162, 95, 200, 204, 250, 255, 100, 48, 191, 194, 219, 32, 149, 164, 51, 106, 206, 18, 8, 227, 174, 225, 148, 242, 136, 24, 77, 125, 136, 6, 154, 184, 89, 207, 104, 199, 212, 24, 71, 21, 189, 71, 25, 248, 55, 252];
        let encoded = bs58::encode(wallet_bytes.clone()).into_string();
        
        println!("Wallet format: {:?}", wallet_bytes);
        println!("Base58 format: {}", encoded);
        
        let decoded = bs58::decode(&encoded).into_vec().unwrap();
        println!("Converted back: {:?}", decoded);
        
        assert_eq!(wallet_bytes, decoded);
        println!("Conversion test passed!");
    }
}
