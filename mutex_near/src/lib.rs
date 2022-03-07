use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::Promise;
use near_sdk::{env, near_bindgen, require, ext_contract, log, AccountId, Gas, PromiseResult};
use near_sdk::PanicOnDefault;
use near_sdk::json_types::{Base64VecU8};
use near_sdk::serde::{Deserialize, Serialize};

const GAS_FOR_FUNCTION_CALL: Gas = Gas(40_000_000_000_000);
const GAS_FOR_CALLBACK: Gas = Gas(5_000_000_000_000);

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MyData{
    s: String,
    i: u64,
    v: Vec<u64>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // SETUP CONTRACT STATE
    md: MyData,
    locker: bool,
}

#[ext_contract(ext_self)]
pub trait ContractCallback{
    fn callback_and_unlock(&mut self);
}


#[near_bindgen]
impl Contract {
    #[init]
    pub fn new()->Self{
        Self{
            md: MyData{
                s: "Hello".to_string(),
                i: 0,
                v: vec![0],
            },
            locker: false,
        }
    }

    // ADD CONTRACT METHODS HERE
    pub fn cross_call_mutex(&mut self, account_id: AccountId, method_name: String, args: String){
        // env::panic_str("we always panic!");

        let arguments = Base64VecU8::from(args.into_bytes());

        let prepaid_gas = env::prepaid_gas();

        self.lock();

        // the state we keep before cross-contract call finished. (That is, we don't unlock before the `callback_and_unlock` completed.)
        self.md.i += 1;
        // Block height can make confusing in `near_sdk_sim`
        // self.md.i = env::block_height().into();
        self.md.v.push(self.md.i);

        Promise::new(account_id)
        .function_call(method_name, 
            arguments.into(), 
            0, 
            GAS_FOR_FUNCTION_CALL)
        .then(ext_self::callback_and_unlock(env::current_account_id(), 0, GAS_FOR_CALLBACK));

        // self.lock();
    }

    pub fn getContext(&self) -> MyData{
        // log!("**********in mutex `getContext`**********");
        self.md.clone()
    }

    #[private]
    pub fn callback_and_unlock(&mut self){
        // unlock first
        self.unlock();

        match env::promise_result(0){
            PromiseResult::Successful(result) =>{
                match near_sdk::serde_json::from_slice::<String>(&result) {
                    Ok(s) => {
                        // cross contract call is completed here.
                        log!("{:#?}", s);
                    }
                    Err(err) => {
                        log!("mutex resolve promise result failed, {}", err);
                    }
                }
            }
            _ =>{
                env::panic_str("in mutex callback! Cross-contract call failed!");
            }
        }
    }

    #[private]
    fn lock(&mut self){
        if (!self.locker){
            self.locker = true;
        }
        else{
            env::panic_str("Locked before!");
        }
    }

    #[private]
    fn unlock(&mut self){
        self.locker = false;
    }

    #[private]
    fn is_locked(&self) ->bool{
        self.locker
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
}
