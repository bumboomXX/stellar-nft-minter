#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_mint_nft_success() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(StellarNFTMinter, ());
    let client = StellarNFTMinterClient::new(&env, &contract_id);

    let owner = Address::generate(&env);

    let nft = client.mint_nft(
        &owner,
        &String::from_str(&env, "Stellar Badge #1"),
        &String::from_str(&env, "My first NFT metadata minted on Stellar Testnet"),
        &String::from_str(&env, "https://placehold.co/600x400"),
        &String::from_str(&env, "https://example.com/stellar-badge-1.json"),
    );

    assert_eq!(nft.token_id, 1);
    assert_eq!(nft.owner, owner);
    assert_eq!(nft.name, String::from_str(&env, "Stellar Badge #1"));
}

#[test]
fn test_get_total_supply() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(StellarNFTMinter, ());
    let client = StellarNFTMinterClient::new(&env, &contract_id);

    let owner = Address::generate(&env);

    client.mint_nft(
        &owner,
        &String::from_str(&env, "Stellar Badge #1"),
        &String::from_str(&env, "First NFT"),
        &String::from_str(&env, "https://placehold.co/600x400"),
        &String::from_str(&env, "https://example.com/stellar-badge-1.json"),
    );

    client.mint_nft(
        &owner,
        &String::from_str(&env, "Stellar Badge #2"),
        &String::from_str(&env, "Second NFT"),
        &String::from_str(&env, "https://placehold.co/600x400"),
        &String::from_str(&env, "https://example.com/stellar-badge-2.json"),
    );

    let total_supply = client.get_total_supply();

    assert_eq!(total_supply, 2);
}

#[test]
fn test_get_owner() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(StellarNFTMinter, ());
    let client = StellarNFTMinterClient::new(&env, &contract_id);

    let owner = Address::generate(&env);

    client.mint_nft(
        &owner,
        &String::from_str(&env, "Stellar Badge #1"),
        &String::from_str(&env, "First NFT"),
        &String::from_str(&env, "https://placehold.co/600x400"),
        &String::from_str(&env, "https://example.com/stellar-badge-1.json"),
    );

    let saved_owner = client.get_owner(&1);

    assert_eq!(saved_owner, owner);
}