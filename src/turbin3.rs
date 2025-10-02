use solana_client::rpc_client::RpcClient;
use solana_system_interface::program as system_program;
use solana_sdk::{
    instruction::{Instruction, AccountMeta},
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
};
use std::str::FromStr;

const RPC_URL: &str = "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5c9c80a";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Add ignore to prevent automatic running
    fn enroll_turbin3() {
        // Step 1: Send enrollment fee to Turbin3 wallet first
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("../../turbin-quest/Turbin3-wallet.json").expect("Couldn't find wallet file");
        
        // Turbin3 enrollment wallet
        let turbin3_wallet = Pubkey::from_str("8wsEeU5EgQ3Thjyvt3odCCPTdf3j8hgyvPvZ1BrLUZTv").unwrap();
        
        // Send 0.1 SOL as enrollment fee
        let transfer_amount = 100_000_000; // 0.1 SOL in lamports
        
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
            
        let transfer_instruction = solana_system_interface::instruction::transfer(
            &signer.pubkey(),
            &turbin3_wallet,
            transfer_amount,
        );
        
        let transaction = Transaction::new_signed_with_payer(
            &[transfer_instruction],
            Some(&signer.pubkey()),
            &[&signer],
            recent_blockhash,
        );
        
        match rpc_client.send_and_confirm_transaction(&transaction) {
            Ok(signature) => {
                println!("Enrollment fee sent successfully!");
                println!("Transaction: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
                println!("Now you can proceed with submit_turbin3()");
            }
            Err(err) => {
                println!("Enrollment failed: {}", err);
            }
        }
    }

    #[test]
    #[ignore] // Add ignore to prevent automatic running
    fn check_pda_status() {
        // Debug function to check PDA account status
        let rpc_client = RpcClient::new(RPC_URL);
        let signer = read_keypair_file("../../turbin-quest/Turbin3-wallet.json").expect("Couldn't find wallet file");
        
        let turbin3_prereq_program = 
            Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        
        // Derive PDA
        let signer_pubkey = signer.pubkey();
        let seeds = &[b"prereqs", signer_pubkey.as_ref()];
        let (prereq_pda, bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);
        
        println!("Wallet address: {}", signer_pubkey);
        println!("PDA address: {}", prereq_pda);
        println!("PDA bump: {}", bump);
        
        // Check PDA
        match rpc_client.get_account(&prereq_pda) {
            Ok(account) => {
                println!("✅ PDA account exists!");
                println!("Account owner: {}", account.owner);
                println!("Account data length: {}", account.data.len());
                println!("Account lamports: {}", account.lamports);
            }
            Err(err) => {
                println!("❌ PDA account does not exist: {}", err);
                println!("This means TypeScript prerequisites haven't been completed yet");
            }
        }
    }

    #[test]
    #[ignore] // Add ignore to prevent automatic running  
    fn submit_rs() {
        // Step 2: Create a Solana RPC client
        let rpc_client = RpcClient::new(RPC_URL);
        
        // Step 3: Load your signer keypair
        let signer = read_keypair_file("../../turbin-quest/Turbin3-wallet.json").expect("Couldn't find wallet file");
        
        // Step 4: Define program and account public keys
        let mint = Keypair::new();
        let turbin3_prereq_program = 
            Pubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();
        let collection = 
            Pubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();
        let mpl_core_program = 
            Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();
        let system_program = system_program::id();
        
        // Step 5: Get the PDA (Program Derived Address)
        let signer_pubkey = signer.pubkey();
        let seeds = &[b"prereqs", signer_pubkey.as_ref()];
        let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);
        
        // Get authority PDA - berdasarkan IDL: ["collection", collection]
        let auth_seeds = &[b"collection", collection.as_ref()];
        let (authority, _auth_bump) = Pubkey::find_program_address(auth_seeds, &turbin3_prereq_program);
        
        // Step 6: Prepare the instruction data (discriminator)
        let data = vec![77, 124, 82, 163, 21, 133, 181, 206];
        
        // Step 7: Define the accounts metadata
        let accounts = vec![
            AccountMeta::new(signer.pubkey(), true),        // user signer
            AccountMeta::new(prereq_pda, false),            // PDA account
            AccountMeta::new(mint.pubkey(), true),          // mint keypair
            AccountMeta::new(collection, false),            // collection
            AccountMeta::new_readonly(authority, false),    // authority (PDA)
            AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
            AccountMeta::new_readonly(system_program, false),   // system program
        ];
        
        // Create the instruction
        let instruction = Instruction {
            program_id: turbin3_prereq_program,
            accounts,
            data,
        };
        
        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        
        // Create and sign transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer, &mint],  // Both signer and mint need to sign
            recent_blockhash,
        );
        
        // Send transaction alert!
        match rpc_client.send_and_confirm_transaction(&transaction) {
            Ok(signature) => {
                println!("Success! Turbin3 prerequisite submitted:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", signature);
                println!("Your completion NFT mint address: {}", mint.pubkey());
            }
            Err(err) => {
                println!("Submission failed: {}", err);
            }
        }
    }
}
