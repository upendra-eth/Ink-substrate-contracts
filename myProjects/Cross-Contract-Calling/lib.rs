#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod caller {

    use base::BaseRef;

    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadLayout,
        },
        Lazy,
    };

   

    #[ink(storage)]
    pub struct Caller {
        base:Lazy<BaseRef>,
        rollno:u8
    }

    impl Caller {
        /// Creates a new flipper smart contract initialized with the given value.
        #[ink(constructor)]
        pub fn new(init_value: bool ,version: u32, base_code_hash: Hash) -> Self {
            
                let total_balance = Self::env().balance();
                let salt = version.to_le_bytes();

                let base = BaseRef::new(init_value)
                .endowment(total_balance / 2)
                .code_hash(base_code_hash)
                .salt_bytes(salt)
                .instantiate()
                .unwrap_or_else(|error| {
                    panic!(
                        "failed at instantiating the Accumulator contract: {:?}",
                        error
                    )
                });

                Self {
                  
                    
                    base: Lazy::new(base),
                    rollno:18,
                    
                }
             }
        

    
        /// Flips the current value of the Flipper's boolean.
        #[ink(message)]
        pub fn change_bool(&mut self) {
            self.base.flip()
        }

        /// Returns the current value of the Flipper's boolean.
        #[ink(message)]
        pub fn get_bool(&self) -> bool {
            self.base.get()
        }

    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            assert!(!flipper.get());
        }

        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert!(!flipper.get());
            flipper.flip();
            assert!(flipper.get());
        }
    }
}
