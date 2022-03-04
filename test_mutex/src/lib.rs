use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, require, ext_contract, near_bindgen, log, Promise, AccountId, PromiseResult, Gas};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{Base64VecU8};
use near_sdk::collections::{LookupMap};
use near_sdk::PanicOnDefault;

const GAS_FOR_FUNCTION_CALL: Gas = Gas(100_000_000_000_000);
const GAS_FOR_CALLBACK: Gas = Gas(5_000_000_000_000);

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MyData{
    s: String,
    i: i32,
    v: Vec<i32>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // SETUP CONTRACT STATE
    lm: LookupMap<i32, String>,
}

#[ext_contract(ext_self)]
pub trait ContractCallback{
    fn callback(&mut self);

    fn callbacl_for_state(&mut self)-> String;
}


#[near_bindgen]
impl Contract {
    #[init]
    pub fn new()-> Self{
        Self{
            lm: LookupMap::new(b"l"),
        }
    } 

    // ADD CONTRACT METHODS HERE
    pub fn say_hello() ->String {
        String::from("hello near!")
    }

    pub fn c_say_hello(&self) -> String{
        String::from("hello near!")
    }

    pub fn visit_state(&mut self) -> Promise{

        // log!("**********in test `visit_state`**********");

        let prepaid_gas = env::prepaid_gas();

        Promise::new(env::predecessor_account_id())
            .function_call("getContext".to_string(), 
                            Vec::new(), 
                            0, 
                            Gas(5_000_000_000_000))
            .then(ext_self::callbacl_for_state(env::current_account_id(), 0, Gas(5_000_000_000_000)))
    }
    
    // cross_call
    pub fn cross_call_test(&mut self, account_id: AccountId, method_name: String, args: String){
        let arguments = Base64VecU8::from(args.into_bytes());

        let prepaid_gas = env::prepaid_gas();

        // make two invacations for one contarct call in one block(exactly in the next block).
        Promise::new(account_id.clone())
        .function_call(method_name.clone(), 
            arguments.clone().into(), 
            0, 
            GAS_FOR_FUNCTION_CALL)
        .and(Promise::new(account_id).function_call(
            method_name,
            arguments.into(), 
            0, 
            GAS_FOR_FUNCTION_CALL))
        .then(ext_self::callback(env::current_account_id(), 0, GAS_FOR_CALLBACK));
    }

    #[private]
    fn p_say_hello(&self) -> String{
        String::from("hello near!")
    }

    #[private]
    pub fn callback(&mut self){
        require!(env::promise_results_count() == 2);
        match env::promise_result(0){
            PromiseResult::Successful(result) =>{
                match near_sdk::serde_json::from_slice::<String>(&result) {
                    Ok(s) => {
                        // cross contract call is completed here.
                        log!("test contract callback ok: {:#?}", s);
                    }
                    Err(err) => {
                        log!("test resolve promise result failed, {}", err);
                    }
                }
            }
            _ =>{
                env::panic_str("in test callback!Cross-contract call failed!");
            }
        }
    }

    #[private]
    pub fn callbacl_for_state(&mut self) -> String{

        // log!("**********in test `callbacl_for_state`**********");

        require!(env::promise_results_count() == 1);

        match env::promise_result(0){
            PromiseResult::Successful(result) =>{
                match near_sdk::serde_json::from_slice::<MyData>(&result) {
                    Ok(s) => {
                        format!("hello near, state:{}", s.i).to_string()
                    }
                    Err(err) => {
                        log!("test resolve promise result failed, {}", err);
                        String::from("hello near error!")
                    }
                }
            }
            _ =>{
                env::panic_str("in test callback!Cross-contract call failed!");
                String::from("hello near error!")
            }
        }
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
    #[test]
    fn my_test0() {
        assert_eq!("hello near!".to_string(), Contract::say_hello());
    }

    #[test]
    fn my_test1(){
        let mut lm = near_sdk::collections::LookupMap::new(b"a");
        let rst0 = lm.insert(&73, &String::from("hello zero"));
        // insert cannot modify the val related to the exsited key in test
        let rst1 = lm.insert(&73, &String::from("hello one"));
        if let Some(val) = rst1 {
            assert_eq!(val, "hello zero".to_string(), "not the value!");
        }else{
            assert!(false, "none of the key!");
        }
        // assert!(lm.contains_key(&77), "none of the key!");
    }
}
