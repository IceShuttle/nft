#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod nft {
    use ink::prelude::string::String;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Nft {
        owner: AccountId,
        price: u32,
        metadata: String,
        creationtime: Timestamp,
        issued_tokens: Mapping<u32, AccountId>,
        total_issued_tokens: u32,
    }

    #[ink(event)]
    pub struct OwnershipTransferred {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
    }
    #[ink(event)]
    pub struct Issue {
        #[ink(topic)]
        to: AccountId,
    }

    impl Nft {
        #[ink(constructor)]
        pub fn new(metadata: String, price: u32) -> Self {
            Self {
                owner: Self::env().caller(),
                price,
                metadata,
                creationtime: Self::env().block_timestamp(),
                issued_tokens: Mapping::new(),
                total_issued_tokens: 0,
            }
        }

        // Read Only functions
        #[ink(message)]
        pub fn get_metadata(&self) -> String {
            self.metadata.clone()
        }
        #[ink(message)]
        pub fn get_price(&self) -> u32 {
            self.price
        }
        #[ink(message)]
        pub fn get_total_issued_tokens(&self) -> u32 {
            self.total_issued_tokens
        }
        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner.clone()
        }
        #[ink(message)]
        pub fn get_creation_timestamp(&self) -> Timestamp {
            self.creationtime
        }
        #[ink(message)]
        pub fn get_token_owner(&self, id: u32) -> Option<AccountId> {
            self.issued_tokens.get(id)
        }
        #[ink(message)]
        pub fn is_valid_token(&self, id: u32) -> bool {
            self.issued_tokens.contains(id)
        }

        //Write functions
        #[ink(message)]
        #[ink(payable)]
        pub fn change_price(&mut self, new_price: u32) {
            if Self::env().caller() == self.owner {
                self.price = new_price;
            } else {
                panic!("Only the legitimate owner of this NFT can change price");
            }
        }

        #[ink(message)]
        #[ink(payable)]
        pub fn issue(&mut self, token_owner: AccountId) -> u32 {
            if Self::env().caller() == self.owner {
                self.issued_tokens
                    .insert(&self.total_issued_tokens, &token_owner);
                self.total_issued_tokens += 1;
                self.env().emit_event(Issue { to: token_owner });
                return self.total_issued_tokens - 1;
            } else {
                panic!("Only the legitimate owner of this NFT can mint NFT");
            }
        }

        #[ink(message)]
        #[ink(payable)]
        pub fn sell(&mut self, token_id: u32, new_owner: AccountId) {
            if Self::env().caller() == self.issued_tokens.get(token_id).unwrap() {
                self.issued_tokens.remove(token_id);
                self.issued_tokens.insert(&token_id, &new_owner);
            } else {
                panic!("Only token owner can sell it");
            }
        }

        #[ink(message)]
        #[ink(payable)]
        pub fn transfer(&mut self, new_owner: AccountId) {
            if Self::env().caller() == self.owner {
                self.owner = new_owner;
                self.env().emit_event(OwnershipTransferred {
                    from: Self::env().caller(),
                    to: new_owner,
                })
            } else {
                panic!("Only the legitimate owner of this NFT can transfer its ownership");
            }
        }
    }
}
