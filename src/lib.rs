use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::{near_bindgen, PanicOnDefault, BorshStorageKey,AccountId, env, Promise, require, PromiseError};
use near_sdk::store::*;
use near_sdk::ext_contract;

#[derive(Debug,BorshSerialize,BorshDeserialize, PanicOnDefault)]
#[near_bindgen]
struct Contract{
    balances: LookupMap<AccountId, u128>,
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey{
    Balances,
}

struct MyU128(u128);

#[near_bindgen]
impl Contract{
    #[init]
    pub fn new() -> Self{
        Self{
            balances: LookupMap::new(StorageKey::Balances),
        }
    }

    pub fn balance_of(&self, account_id: AccountId) -> U128{
        U128(*self.balances.get(&account_id).unwrap_or(&0u128))
    }

    #[payable]
    pub fn mint(&mut self){
        let account_id: AccountId = env::predecessor_account_id();
        let amount: u128 = env::attached_deposit();
        let current_balance: u128 = self.balance_of(account_id.clone()).0;
        self.balances
            .set(account_id, Some(current_balance + amount));
    }

    pub fn burn(&mut self, amount:U128) -> Promise{
        let account_id: AccountId = env::predecessor_account_id();
        let current_balance: u128 = self.balance_of(account_id.clone()).0;
        self.balances
            .set(account_id.clone(), Some(current_balance.checked_sub(amount.0)
            .unwrap_or_else(||{
                env::panic_str("You tried to withdraw more tokens than you own.")
            }))
        );

        // let _ = env::current_account_id(); // the account that the contract is deployed to
        // let _ = env::predecessor_account_id(); // the account that called this contract (inmediately preceding this call)
        // let _ = env::signer_account_id(); // the account that signed the transaction that iniated this entire call chain 

        Promise::new(account_id).transfer(amount.0)
    }

    pub fn transfer(&mut self, receiver_id: AccountId, amount: U128){
        let sender_id = env::predecessor_account_id();

        require!(
            sender_id != receiver_id,
            "Sender and reciver must be different",
        );

        let sender_current_balance: u128 = self.balance_of(sender_id.clone()).0;
        let receiver_current_balance: u128 = self.balance_of(receiver_id.clone()).0;

        let sender_new_balance = sender_current_balance
            .checked_sub(amount.0).
            unwrap_or_else(||{
                env::panic_str("You tried to send more token than you own!")
            });
        let receiver_new_balance: u128 = receiver_current_balance
            .checked_add(amount.0)
            .unwrap_or_else(|| env::panic_str("Your balance overflowed!"));
        
        self.balances.set(sender_id, Some(sender_new_balance));
        self.balances.set(receiver_id, Some(receiver_new_balance));
    }

    pub fn call_nft_contract(&self, contract_id: AccountId) -> Promise{
        // Promise::new(contract_id).function_call("nft_token".to_string(), json!({"token_id": "token_0"}).to_string().into_bytes(),0,);
        nft_contract::ext(contract_id).nft_token("token_0".to_string()).then(Self::ext
            ())
    }

    #[private]
    pub fn after_calling_nft_contract(&self, #[callback_result]result: Result<String, PromiseError>){
        //other stuff here
        match result{
            Ok(success_result: String) =>{},
            Error(error_result: PromiseError) => {},
        }
    }
}

#[ext_contract(nft_contract)]
trait NftContract{
    fn nft_token(token_id: String) -> String;
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test(){}
}

