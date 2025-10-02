# Turbin3 Prerequisites - Rust

Implementasi Rust untuk menyelesaikan prerequisites Turbin3. Project ini cover semua yang dibutuhkan mulai dari generate wallet sampai submit ke program Turbin3 pake raw instructions.

## What's Inside

- `keygen.rs` - Generate keypair baru
- `converter.rs` - Convert format wallet (Solana ↔ Phantom Base58)
- `airdrop.rs` - Claim SOL devnet
- `transfer.rs` - Transfer SOL antar wallet
- `turbin3.rs` - Submit ke Turbin3 program

## Status

✅ TypeScript prerequisites done  
✅ Rust submission berhasil  
✅ TX: `XafLT1YPMoNYVFzAK7heSwR8in6b8w5t4qz3f79FHMVB3n5Z8ZE5u4fWC4GQzMFNPqsHxqbrHVzjN6DtKuZ1UpJ`

## Quick Start

```bash
# Generate wallet
cargo test keygen -- --show-output

# Claim airdrop
cargo test claim_airdrop -- --show-output

# Transfer SOL
cargo test transfer_sol -- --show-output

# Submit ke Turbin3 (butuh TypeScript prerequisites dulu)
cargo test submit_rs -- --ignored --nocapture
```

## Dependencies

```toml
solana-sdk = "3.0.0"
solana-client = "3.0.3"
solana-program = "3.0.0"
solana-system-interface = "2.0.0"
bs58 = "0.5.1"
```

## Turbin3 Setup

Program ID: `TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM`  
Collection: `5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2`  
Network: Devnet

### submit_rs Flow

1. Load wallet yang sama dengan TypeScript prerequisites
2. Derive PDA dengan seeds `["prereqs", user_pubkey]`
3. Build instruction dengan discriminator `[77, 124, 82, 163, 21, 133, 181, 206]`
4. Submit transaction

## Notes

- Harus pake wallet yang sama untuk TypeScript dan Rust steps
- TypeScript prerequisites wajib diselesaikan dulu (buat initialize PDA)
- Path wallet: `../../turbin-quest/Turbin3-wallet.json`

## Troubleshooting

**AccountNotInitialized Error?**  
→ Selesaikan TypeScript prerequisites dulu

**Wrong PDA?**  
→ Pastikan pake wallet yang sama dan seeds yang benar

## Result

Berhasil submit Rust prerequisites dan dapet NFT mint sebagai proof of completion. Ready for next phase Turbin3 Builders Cohort! 🚀

## �️ Turbin3 Submission Process

### Prerequisites Workflow

1. **TypeScript Prerequisites** (✅ Completed)

   - Initialize PDA account using TypeScript client
   - Submit completion proof with `submit_ts` instruction
   - Creates NFT mint for verification

2. **Rust Prerequisites** (✅ Completed)
   - Load same wallet used in TypeScript step
   - Derive PDA using seeds: `["prereqs", user_pubkey]`
   - Submit using `submit_rs` instruction with discriminator: `[77, 124, 82, 163, 21, 133, 181, 206]`

### Raw Instruction Details

The Rust submission uses raw Solana instructions:

```rust
// PDA derivation
let seeds = &[b"prereqs", signer_pubkey.as_ref()];
let (prereq_pda, _bump) = Pubkey::find_program_address(seeds, &turbin3_prereq_program);

// Authority PDA for collection
let auth_seeds = &[b"collection", collection.as_ref()];
let (authority, _auth_bump) = Pubkey::find_program_address(auth_seeds, &turbin3_prereq_program);

// Instruction data (submit_rs discriminator)
let data = vec![77, 124, 82, 163, 21, 133, 181, 206];
```

### Account Structure

Required accounts for `submit_rs` instruction:

- `user` (signer, writable) - The wallet submitting
- `account` (PDA, writable) - The prerequisite account
- `mint` (signer, writable) - New NFT mint address
- `collection` (writable) - Turbin3 collection
- `authority` (PDA) - Collection authority
- `mpl_core_program` - Metaplex Core program
- `system_program` - Solana system program

## 🎓 Learning Objectives

This project demonstrates:

- **Raw Solana Instructions** - Building instructions without client libraries
- **PDA Derivation** - Program Derived Address creation and usage
- **Multi-step Workflows** - Coordinating TypeScript and Rust implementations
- **Account Management** - Working with signers, PDAs, and program accounts
- **Error Handling** - Debugging common Solana program errors
- **Transaction Construction** - Building, signing, and sending transactions
- **RPC Interactions** - Communicating with Solana network via RPC
- RPC client usage for blockchain interactions
- Transaction creation and signing
- Program Derived Address (PDA) generation
- Smart contract interaction patterns
- Error handling in blockchain applications

## 🔒 Security Notes

- Never commit private keys to version control
- Use devnet for testing only
- Keep private keys secure in production environments
- The `dev-wallet.json` should be added to `.gitignore`

## 📚 Resources

- [Solana Documentation](https://docs.solana.com/)
- [Turbin3 Program](https://explorer.solana.com/address/TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM?cluster=devnet)
- [Solana Rust SDK](https://docs.rs/solana-sdk/)

## 🤝 Contributing

This is a learning project for Turbin3 prerequisites. Feel free to fork and experiment with the code for educational purposes.

## 📄 License

This project is for educational purposes as part of the Turbin3 prerequisites program.

## ⚠️ Important Notes

### Wallet Requirements

- **Critical**: Must use the same wallet for both TypeScript and Rust steps
- TypeScript prerequisites initialize the PDA account
- Rust submission requires the initialized PDA to exist
- Path used: `../../turbin-quest/Turbin3-wallet.json`

### Common Issues

**AccountNotInitialized Error**

```
Error Code: AccountNotInitialized. Error Number: 3012
```

**Solution**: Complete TypeScript prerequisites first to initialize PDA

**Wrong PDA Derivation**

- Ensure seeds are correct: `["prereqs", user_pubkey]`
- Authority PDA uses: `["collection", collection_pubkey]`
- Use same wallet address for derivation

### Transaction Explorer

All transactions can be viewed on Solana Explorer:

- **Network**: Devnet
- **Base URL**: `https://explorer.solana.com/tx/{signature}?cluster=devnet`

### Success Verification

Upon successful submission, you'll receive:

- ✅ Transaction signature
- ✅ NFT mint address
- ✅ Explorer link for verification

## 🎊 Congratulations!

You have successfully completed the Turbin3 Solana Prerequisites in Rust!

This demonstrates mastery of:

- Solana program interactions using raw instructions
- PDA derivation and account management
- Multi-language blockchain development workflow
- Transaction construction and error handling

Ready for the next phase of your Turbin3 Builder's journey! 🚀
