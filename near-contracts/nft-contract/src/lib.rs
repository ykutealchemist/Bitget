use std::collections::HashMap;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, CryptoHash, PanicOnDefault, Promise, PromiseOrValue,
};

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::mint::*;
pub use crate::nft_core::*;
pub use crate::approval::*;
pub use crate::royalty::*;
pub use crate::artist::*;
pub use crate::events::*;

mod internal;
mod approval; 
mod enumeration; 
mod metadata; 
mod mint; 
mod nft_core; 
mod royalty;
mod artist; 
mod events;

pub const NFT_METADATA_SPEC: &str = "nft-1.0.0"; /// NFT Standard Version
pub const NFT_STANDARD_NAME: &str = "nep171"; /// NFT Standard Name

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // Contract Owner
    pub owner_id: AccountId,

    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>, //  Keeps track of Token IDs for a given account
    pub tokens_per_search: UnorderedMap<SearchTerm, UnorderedSet<TokenId>>, // Keep track of Token IDs for a given search term
    pub tokens_by_id: LookupMap<TokenId, Token>, // Keep track of Token struct for a given Token ID

    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>, // Keeps track of Token metadata for a given Token ID

    pub artist_by_account: LookupMap<AccountId, Artist>, // Keep track of the Artist Name for a given Account ID

    // Keeps track of Metadata for THIS contract
    pub metadata: LazyOption<NFTContractMetadata>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    TokensPerOwner,
    TokenPerOwnerInner { account_id_hash: CryptoHash },
    TokensPerSearch,
    TokensPerSearchInner { search_term_hash: CryptoHash  },
    TokensById,
    TokenMetadataById,
    ArtistByAccount,
    NFTContractMetadata,
    TokensPerType,
    TokensPerTypeInner { token_type_hash: CryptoHash },
    TokenTypesLocked,
}

#[near_bindgen]
impl Contract {
    /*
        initialization function (can only be called once).
        this initializes the contract with default metadata so the
        user doesn't have to manually type metadata.
    */
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        // Calls the "new" function with some default metadata and the owner_id passed in 
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "Nearsound NFT".to_string(),
                symbol: "NS".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    /*
        initialization function (can only be called once).
        this initializes the contract with metadata that was passed in and
        the owner_id. 
    */
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        // Create a variable of type Self with all the fields initialized. 
        let this = Self {
            // Storage keys are simply the prefixes used for the collections. This helps avoid data collision.
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner.try_to_vec().unwrap()),
            tokens_per_search: UnorderedMap::new(StorageKey::TokensPerSearch.try_to_vec().unwrap()),
            tokens_by_id: LookupMap::new(StorageKey::TokensById.try_to_vec().unwrap()),
            token_metadata_by_id: UnorderedMap::new(
                StorageKey::TokenMetadataById.try_to_vec().unwrap(),
            ),
            artist_by_account: LookupMap::new(
                StorageKey::ArtistByAccount.try_to_vec().unwrap(),
            ),
            // Set the owner_id field equal to the passed in owner_id. 
            owner_id,
            metadata: LazyOption::new(
                StorageKey::NFTContractMetadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
        };

        //return the Contract object
        this
    }
}