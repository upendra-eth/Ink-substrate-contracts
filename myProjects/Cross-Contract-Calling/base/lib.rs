#![cfg_attr(not(feature = "std"), no_std)]

pub use self::base::{
    Base,
    BaseRef,
};

use ink_lang as ink;

#[ink::contract]
pub mod base{

    #[ink(storage)]
    pub struct Base {
        value: bool,
    }

    impl Base {
        /// Creates a new flipper smart contract initialized with the given value.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Creates a new flipper smart contract initialized to `false`.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Flips the current value of the Flipper's boolean.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Returns the current value of the Flipper's boolean.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}