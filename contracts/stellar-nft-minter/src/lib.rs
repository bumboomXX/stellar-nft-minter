#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, symbol_short, Address,
    Env, String,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NFT {
    pub token_id: u32,
    pub owner: Address,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub metadata_url: String,
    pub minted_at: u64,
}

#[contracttype]
pub enum DataKey {
    NextTokenId,
    NFT(u32),
    Owner(u32),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum NFTError {
    InvalidName = 1,
    InvalidMetadata = 2,
    NFTNotFound = 3,
    NotOwner = 4,
}

#[contract]
pub struct StellarNFTMinter;

#[contractimpl]
impl StellarNFTMinter {
    pub fn mint_nft(
        env: Env,
        owner: Address,
        name: String,
        description: String,
        image_url: String,
        metadata_url: String,
    ) -> NFT {
        owner.require_auth();

        if name.len() == 0 {
            panic_with_error!(&env, NFTError::InvalidName);
        }

        if metadata_url.len() == 0 {
            panic_with_error!(&env, NFTError::InvalidMetadata);
        }

        let token_id: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::NextTokenId)
            .unwrap_or(1);

        let nft = NFT {
            token_id,
            owner: owner.clone(),
            name,
            description,
            image_url,
            metadata_url,
            minted_at: env.ledger().timestamp(),
        };

        env.storage()
            .persistent()
            .set(&DataKey::NFT(token_id), &nft);

        env.storage()
            .persistent()
            .set(&DataKey::Owner(token_id), &owner);

        env.storage()
            .persistent()
            .set(&DataKey::NextTokenId, &(token_id + 1));

        env.events()
            .publish((symbol_short!("minted"), owner), token_id);

        nft
    }

    pub fn get_nft(env: Env, token_id: u32) -> NFT {
        load_nft(&env, token_id)
    }

    pub fn get_owner(env: Env, token_id: u32) -> Address {
        match env.storage().persistent().get(&DataKey::Owner(token_id)) {
            Some(owner) => owner,
            None => panic_with_error!(&env, NFTError::NFTNotFound),
        }
    }

    pub fn get_total_supply(env: Env) -> u32 {
        let next_id: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::NextTokenId)
            .unwrap_or(1);

        next_id - 1
    }

    pub fn verify_owner(env: Env, owner: Address, token_id: u32) -> bool {
        let nft = load_nft(&env, token_id);

        if nft.owner != owner {
            panic_with_error!(&env, NFTError::NotOwner);
        }

        true
    }
}

fn load_nft(env: &Env, token_id: u32) -> NFT {
    match env.storage().persistent().get(&DataKey::NFT(token_id)) {
        Some(nft) => nft,
        None => panic_with_error!(env, NFTError::NFTNotFound),
    }
}