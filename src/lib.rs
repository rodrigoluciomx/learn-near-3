use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault, BorshStorageKey};
use near_sdk::store::*;

#[derive(Debug,BorshSerialize,BorshDeserialize, PanicOnDefault)]
#[near_bindgen]
struct Contract{
    balances: LookupMap<>
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey{
    Vector,
}

#[near_bindgen]
impl Contract{
    #[init]
    pub fn new() -> Self{
        Self{
            number: 0,
            list: Vector::new(StorageKey::Vector),
        }
    }

    pub fn get_number(&self) -> u8{
        self.number
    }

    pub fn increment(&mut self){
        self.number += 1;
    }
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test(){}
}

