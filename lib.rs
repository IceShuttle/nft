#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod nft {
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct Nft {
        owner: AccountId,
        price: u32,
        metadata: String,
        creationtime: Timestamp,
    }

    #[ink(event)]
    pub struct OwnershipTransferred {
        #[ink(topic)]
        from: AccountId,
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
        pub fn get_owner(&self) -> AccountId {
            self.owner.clone()
        }

        #[ink(message)]
        pub fn get_timestamp(&self) -> Timestamp {
            self.creationtime
        }

        //Write functions
        #[ink(message)]
        pub fn change_price(&mut self, new_price: u32) {
            if Self::env().caller() == self.owner {
                self.price = new_price;
            } else {
                panic!("Only the legitimate owner of this NFT can change price");
            }
        }

        #[ink(message)]
        #[ink(payable)]
        pub fn drop(&mut self) {}

        #[ink(message)]
        pub fn transfer(&mut self, new_owner: AccountId) {
            if Self::env().caller() == self.owner {
                self.owner = new_owner;
                self.env().emit_event(OwnershipTransferred {
                    from: Self::env().caller(),
                    to: new_owner,
                })
            } else {
                panic!("Only the legitimate owner of this NFT can sell this");
            }
        }
    }
}
