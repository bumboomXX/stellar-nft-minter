# Stellar NFT Minter

## Project Overview

Stellar NFT Minter is a Level 3 Stellar mini-dApp built for the monthly challenge.

The project allows users to mint NFT metadata on Stellar Testnet using a Soroban smart contract.  
Each NFT record stores basic metadata such as owner address, name, description, image URL, metadata URL, token ID, and minted timestamp.

This project focuses on building a complete end-to-end mini-dApp with smart contract interaction, testing, documentation, screenshots, and demo video.

---

## Project Vision

The long-term vision of this project is to create a beginner-friendly NFT minting experience on Stellar.

Many new users find NFT minting difficult because they need to understand wallets, metadata, transaction signing, and blockchain explorers. This project simplifies the flow by allowing users to mint and track NFT metadata through a simple smart contract.

In the current version, the project works as an on-chain NFT metadata registry. It does not yet implement a full NFT token standard, but it demonstrates the core logic needed for NFT minting:

- connecting an owner wallet
- submitting NFT metadata
- storing NFT data on-chain
- reading NFT information by token ID
- checking NFT ownership
- tracking total NFT supply

In future versions, this can evolve into a complete NFT minting platform for badges, community collectibles, certificates, or proof-of-participation assets on Stellar.

---

## Level 3 Requirements

| Requirement | Status |
|---|---|
| Mini-dApp fully functional | Completed |
| Minimum 3 tests passing | Completed |
| README complete | Completed |
| Demo video recorded | Completed |
| Minimum 3+ meaningful commits | Completed |
| Public GitHub repository | Completed |

---
##screenshots:
![screenshots](test1.png)
<img width="1365" height="644" alt="image" src="https://github.com/user-attachments/assets/642061a2-cc56-4997-ac4e-9828b7358992" />

## Deployed Contract Details

Network: Stellar Testnet

Contract Name: StellarNFTMinter

Contract Address: CBWRZNDN4BZCROW43OCFRCB6EEVDSSQSOP335NT2IFY6YHLBKRI6AQU2W

Source Account: GCSGI4ZRPWFZV3DZMHNRELFJXJ6YELBV3LDADK7AWUHSELBLPAU4UT42

The deployed contract stores NFT metadata records on Stellar Testnet. It allows users to mint NFT metadata, read NFT data, check ownership, and track total supply.
## Key Features

- Mint NFT metadata on Stellar Testnet
- Store NFT owner, name, description, image URL, and metadata URL
- Generate a unique `token_id` for each NFT
- Read NFT details by token ID
- Read NFT owner by token ID
- Track total NFT supply
- Emit a `minted` event when a new NFT is created
- Handle basic contract errors
- Include 3 passing smart contract tests
- Include documentation, screenshots, and demo video

---

## Smart Contract Functions

| Function | Type | Description |
|---|---|---|
| `mint_nft` | Write | Mints a new NFT metadata record |
| `get_nft` | Read-only | Returns full NFT information by token ID |
| `get_owner` | Read-only | Returns the owner address of a token ID |
| `get_total_supply` | Read-only | Returns the total number of NFTs minted |
| `verify_owner` | Read-only | Checks whether an address owns a specific NFT |

---

## NFT Data Structure

Each NFT contains:

| Field | Description |
|---|---|
| `token_id` | Unique NFT ID |
| `owner` | Wallet address that minted the NFT |
| `name` | NFT name |
| `description` | NFT description |
| `image_url` | Image URL for the NFT |
| `metadata_url` | External metadata URL |
| `minted_at` | Ledger timestamp when the NFT was minted |

Example NFT:

```text
token_id: 1
owner: GCSGI4...
name: Stellar Badge #1
description: My first NFT metadata minted on Stellar Testnet
image_url: https://placehold.co/600x400
metadata_url: https://example.com/stellar-badge-1.json

## Future Scope

This Level 3 version focuses on building a functional NFT metadata minter with smart contract interaction, tests, documentation, screenshots, and a demo video.

Planned improvements include:

- Full NFT standard support on Stellar
- Real metadata hosting with IPFS, Arweave, or another decentralized storage provider
- NFT gallery to display all NFTs minted by a wallet
- Multi-wallet support with StellarWalletsKit
- NFT transfer function between wallet addresses
- Better frontend experience with loading states, progress indicators, success badges, error messages, explorer links, and cached NFT history
