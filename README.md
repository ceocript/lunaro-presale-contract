
# Lunaro Token Presale Smart Contract

This is the official Anchor-based smart contract for the Lunaro ($LNR) token presale on the Solana blockchain.

## Features
- Fixed exchange: 1 SOL = 1000 LNR
- Cap at $35 million in SOL equivalent
- SOL sent to Squads multisig wallet
- Buyer information tracked on-chain

## Deploy Instructions
1. Install Anchor CLI: https://book.anchor-lang.com/
2. `anchor build`
3. `anchor deploy --provider.cluster devnet`

## Multisig & Vault
Use Squads protocol to manage presale funds securely.

## License
MIT
